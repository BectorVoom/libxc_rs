//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1315/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1315(t114: f64, t25826: f64, t75833: f64, t22628: f64, t6998: f64, t101451: f64, t105870: f64, t105878: f64, t114394: f64, t94974: f64, t1312: f64, t105866: f64, t114360: f64, t114363: f64, t114372: f64, t114375: f64, t114377: f64, t114380: f64, t114382: f64, t114384: f64, t114387: f64, t114389: f64, t114391: f64, t1518: f64, t22633: f64, t28030: f64, t33602: f64, t5920: f64, t6985: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t114396 = t25826 * t75833;
    let t114398 = t6998 * t22628;
    let t114401 = piecewise3(t115, 0.0_f64, -t94974 - 11.0_f64 / 3.0_f64 * t101451 - 2.0_f64 * t105870 + t105878 - 3.0_f64 / 4.0_f64 * t114394 + 3.0_f64 / 4.0_f64 * t114396 - t114398 / 8.0_f64);
    let t114403 = 2.0_f64 * t1312 * t114401;
    let t114404 = 6.0_f64 * t105866 * t1518 + 2.0_f64 * t22633 * t6985 + 6.0_f64 * t28030 * t5920 + 6.0_f64 * t33602 * t5920 + t114360 + 6.0_f64 * t114363 + t114372 + t114375 + t114377 + t114380 + t114382 + t114384 + t114387 + t114389 + t114391 + t114403;
    (t114401, t114404)
}
