//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 850/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk850<F: Float>(t7967: F, t9713: F, t1072: F, t1074: F, t2387: F, t1069: F, t2489: F, t102: F, t818: F, t329: F, t3407: F, t3403: F) -> (F, F, F, F, F, F) {
    let t9714 = t7967 * t9713;
    let t9717 = t2387 * t1072 * t1074;
    let t9719 = t1069 * t2489;
    let t9721 = t102 * t818;
    let t9722 = t9721 * t329;
    let t9723 = t9722 * t3407;
    let t9724 = t3403 * t9723;
    (t9714, t9717, t9719, t9722, t9723, t9724)
}
