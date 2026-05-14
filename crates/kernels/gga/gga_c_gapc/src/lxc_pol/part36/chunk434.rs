//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 434/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk434<F: Float>(t2520: F, t612: F, t1936: F, t889: F, t6: F, t891: F, t2503: F, t1944: F, t320: F, t1: F, t314: F) -> (F, F, F, F, F) {
    let t2521 = t2520 * t612;
    let t2524 = t889 * t1936;
    let t2525 = t891 * t6;
    let t2526 = t2503 * t2525;
    let t2529 = t320 * t1944;
    let t2530 = t314 * t1;
    (t2521, t2524, t2526, t2529, t2530)
}
