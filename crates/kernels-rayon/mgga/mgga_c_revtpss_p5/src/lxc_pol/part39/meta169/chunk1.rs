//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 751/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk751(t1214: f64, t1248: f64, t1250: f64, t3720: f64, t1222: f64, t1235: f64, t1238: f64, t1252: f64, t3663: f64, t3667: f64, t3671: f64, t3674: f64, t3679: f64, t3684: f64, t3686: f64, t3689: f64, t3694: f64, t3701: f64, t3705: f64, t3708: f64, t3711: f64, t3714: f64, t3718: f64) -> (f64, f64, f64, f64) {
    let t3721 = t1214 * t1248;
    let t3722 = t3721 * t1250;
    let t3723 = t3720 * t3722;
    let t3726 = -0.21437009059034868486e-3_f64 * t1235 * t3663 - 0.42874018118069736972e-3_f64 * t3667 * t1238 + 0.42874018118069736972e-3_f64 * t3671 * t3674 - 0.28582678745379824648e-3_f64 * t3679 - t3684 - t3686 / 432.0_f64 - t1222 * t3689 / 288.0_f64 - t1222 * t3694 / 144.0_f64 + t1222 * t3701 / 216.0_f64 + 0.28582678745379824648e-3_f64 * t3705 + 0.42874018118069736972e-3_f64 * t3708 * t1252 + 0.28582678745379824648e-3_f64 * t3711 * t3714 - 0.42874018118069736972e-3_f64 * t3718 * t3723;
    (t3721, t3722, t3723, t3726)
}
