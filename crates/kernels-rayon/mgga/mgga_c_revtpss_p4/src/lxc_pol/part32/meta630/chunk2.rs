//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2033/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2033(t110698: f64, t892: f64, t198: f64, t205: f64, t8019: f64, t102854: f64, t105906: f64, t106534: f64, t106540: f64, t106546: f64, t106562: f64, t106590: f64, t106593: f64, t106606: f64, t1468: f64, t1940: f64, t26425: f64, t26585: f64, t26590: f64, t27160: f64, t28291: f64, t28456: f64, t28472: f64, t29599: f64, t29719: f64, t30: f64, t7432: f64, t7787: f64, t95511: f64) -> (f64, f64, f64) {
    let t110699 = t110698 * t892;
    let t110704 = t198 * t205 * t8019;
    let t110711 = -t1940 * t26585 * t29719 / 2.0_f64 - t1940 * t7432 * t106606 / 2.0_f64 - 6.0_f64 * t28291 * t106534 - 3.0_f64 * t26425 * t105906 + t1940 * t26590 * t106593 - t1940 * t102854 * t7787 + 3.0_f64 * t26425 * t106562 - 3.0_f64 / 2.0_f64 * t26425 * t106540 + 2.0_f64 * t28472 * t106590 + 6.0_f64 * t28291 * t106546 + t1940 * t110699 * t30 / 2.0_f64 + 6.0_f64 * t110704 * t27160 + t1940 * t28456 * t1468 - 3.0_f64 * t95511 * t29599;
    (t110699, t110704, t110711)
}
