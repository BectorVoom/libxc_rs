//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 877/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk877<F: Float>(t11451: F, t5126: F, t11450: F, t1936: F, t5462: F, t144: F, t1453: F, t5526: F, t674: F, t5708: F, t612: F, t5713: F, t9066: F, t3060: F, t3687: F, t1040: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11452 = t11451 * t5126;
    let t11453 = t11450 * t11452;
    let t11455 = t5462 * t1936;
    let t11456 = t1453 * t144;
    let t11458 = t11456 * t674 * t5526;
    let t11459 = t11455 * t11458;
    let t11463 = t5708 * t612;
    let t11465 = t9066 * t144 * t5713;
    let t11466 = t11463 * t11465;
    let t11468 = t3060 * t3687;
    let t11469 = t11468 * t1040;
    (t11452, t11453, t11455, t11458, t11459, t11463, t11465, t11466, t11468, t11469)
}
