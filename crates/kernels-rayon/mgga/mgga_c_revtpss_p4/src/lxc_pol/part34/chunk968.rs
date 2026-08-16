//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 968/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk968(t150: f64, t23210: f64, t190: f64, t1469: f64, t18305: f64, t4401: f64, t14613: f64, t6002: f64, t22671: f64, t706: f64, t10592: f64, t10596: f64, t10604: f64, t10611: f64, t23193: f64, t23213: f64, t9542: f64) -> (f64, f64, f64, f64, f64) {
    let t23214 = t150 * t23210;
    let t23215 = t23214 * t190;
    let t23216 = t18305 * t1469;
    let t23218 = 36.0_f64 * t4401 * t23216;
    let t23220 = 36.0_f64 * t14613 * t6002;
    let t23221 = t190 * t22671;
    let t23223 = 4.0_f64 * t706 * t23221;
    let t23224 = t10592 + t23193 - t10596 - t10604 + t23213 + t23215 + t9542 + t23218 + t23220 - t10611 + t23223;
    (t23215, t23218, t23220, t23223, t23224)
}
