//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3034/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3034<F: Float>(t10505: F, t137: F, t15002: F, t41011: F, t11015: F, t4325: F, t4477: F, t9292: F, t14472: F, t2439: F, t887: F, t14979: F, t689: F, t779: F) -> (F, F, F, F, F) {
    let t51207 = t41011 * t15002 * t137 * t10505;
    let t51211 = t4325 * t11015;
    let t51213 = t9292 * t4477;
    let t51216 = t2439 * t14472 * t887;
    let t51227 = t689 * t779 * t14979;
    (t51207, t51211, t51213, t51216, t51227)
}
