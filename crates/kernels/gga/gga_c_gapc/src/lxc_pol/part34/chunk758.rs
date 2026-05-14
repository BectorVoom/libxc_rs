//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 758/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk758<F: Float>(t8700: F, t889: F, t3397: F, t1068: F, t2387: F, t322: F, t3307: F, t913: F, t3288: F, t7577: F, t3303: F, t3300: F, t7553: F, t3012: F, t7557: F, t2578: F) -> (F, F, F, F, F, F, F) {
    let t9512 = t889 * t8700;
    let t9513 = t9512 * t3397;
    let t9515 = t2387 * t1068;
    let t9516 = t9515 * t322;
    let t9518 = t3307 * t913;
    let t9520 = t3288 * t7577;
    let t9521 = t3303 * t9520;
    let t9523 = t7553 * t3300;
    let t9525 = t3012 * t7557;
    let t9526 = t2578 * t9525;
    (t9513, t9516, t9518, t9520, t9521, t9523, t9526)
}
