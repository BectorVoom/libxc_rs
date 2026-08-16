//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 993/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk993(t10673: f64, t10687: f64, t10692: f64, t10870: f64, t10900: f64, t14712: f64, t14716: f64, t14761: f64, t14765: f64, t18338: f64, t18340: f64, t23253: f64, t23257: f64, t23263: f64, t23267: f64, t23275: f64, t2721: f64, t2730: f64, t799: f64) -> f64 {
    let t23278 = t10673 - 0.12862205435420921092e-2_f64 * t10870 * t23253 + 0.12862205435420921092e-2_f64 * t2721 * t23257 - 0.17006693853500995666e-1_f64 * t14712 + 0.40656002247428262579e-3_f64 * t14716 - t10900 * t23263 / 4.0_f64 - t799 * t23267 / 48.0_f64 - 0.13553694749236397037e-4_f64 * t14761 - t10687 + t10692 - 35.0_f64 / 72.0_f64 * t14765 + 7.0_f64 / 48.0_f64 * t18338 - 7.0_f64 / 16.0_f64 * t18340 + 3.0_f64 / 16.0_f64 * t2730 * t23275;
    t23278
}
