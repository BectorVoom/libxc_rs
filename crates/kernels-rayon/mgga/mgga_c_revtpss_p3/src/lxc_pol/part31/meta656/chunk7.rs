//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2212/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2212(t6864: f64, t94455: f64, t26024: f64, t6846: f64, t102529: f64, t102549: f64, t94484: f64, t94498: f64, t98222: f64, t98227: f64, t98230: f64, t98236: f64, t98239: f64, t98244: f64, t98259: f64) -> f64 {
    let t108590 = t94455 * t6864;
    let t108592 = t26024 * t6846;
    let t108596 = -0.40015750243531754507e-2_f64 * t108590 + 0.20007875121765877254e-2_f64 * t108592 - 0.80031500487063509015e-1_f64 * t98222 - t102529 + t98227 - t98230 - t98236 + t98239 + t94484 + t98244 + 0.27104001498285508387e-3_f64 * t94498 - t98259 - t102549;
    t108596
}
