//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 959/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk959<F: Float>(t25560: F, t8207: F, t769: F, t935: F, t3916: F, t530: F, t864: F, t3881: F, t287: F, t320: F, t321: F, t3695: F, t2663: F, t276: F, t308: F, t115: F, t282: F) -> (F, F, F, F, F, F, F, F) {
    let t25561 = t8207 * t25560;
    let t25564 = t935 * t769;
    let t25570 = t3916 * t25560;
    let t25622 = t530 * t864;
    let t25742 = t3881 * t25560;
    let t25788 = 0.85858385084333410912e-1 * t320 * t321 * t3695 * t287;
    let t25834 = 1.0 / t2663 / t308 / t276;
    let t25836 = t282 * t25834 * t115;
    (t25561, t25564, t25570, t25622, t25742, t25788, t25834, t25836)
}
