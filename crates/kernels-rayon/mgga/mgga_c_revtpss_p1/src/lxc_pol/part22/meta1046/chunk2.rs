//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3673/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3673(t20520: f64, t3479: f64, t1168: f64, t12418: f64, t12423: f64, t12429: f64, t12470: f64, t12472: f64, t12511: f64, t17086: f64, t20521: f64, t20615: f64, t20618: f64, t20619: f64, t20625: f64, t3447: f64, t3452: f64, t3453: f64, t3471: f64, t3477: f64, t45085: f64, t45194: f64, t5120: f64, t6487: f64, t6502: f64, t6503: f64, t6506: f64, t68956: f64, t68961: f64, t68963: f64, t68965: f64, t68967: f64) -> f64 {
    let t69411 = t20520 * t3479;
    let t69422 = -t68956 + t68961 + t68963 - t68965 - t68967 + 2.0_f64 * t5120 * t17086 - 2.0_f64 * t45194 * t6487 + 1.0_f64 * t12418 * t6503 + 2.0_f64 * t3447 * t20521 - 0.19298375398431042081e3_f64 * t12429 * t6506 * t3471 - 0.24828486201251232145e5_f64 * t45085 * t20625 * t3453 - 4.0_f64 * t12511 * t20615 - 4.0_f64 * t3452 * t20521 * t1168 - 2.0_f64 * t3452 * t6503 * t3471 - 0.19298375398431042081e3_f64 * t12429 * t20618 * t3453 + 0.64327917994770140268e2_f64 * t12423 * t20619 + 0.64327917994770140268e2_f64 * t3477 * t69411 * t1168 + 0.32163958997385070134e2_f64 * t3477 * t20618 * t3471 + 0.2069040516770936012e4_f64 * t12470 * t6502 * t12472 * t3453;
    t69422
}
