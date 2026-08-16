//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1497/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1497(t114: f64, t118655: f64, t118688: f64, t118728: f64, t118746: f64, t1312: f64, t13426: f64, t1453: f64, t18227: f64, t18245: f64, t2322: f64, t27123: f64, t27126: f64, t28219: f64, t30143: f64, t31382: f64, t31407: f64, t31459: f64, t31653: f64, t31654: f64, t31660: f64, t4248: f64, t4254: f64, t5517: f64, t5523: f64, t569: f64, t651: f64, t7732: f64, t7889: f64, t8325: f64, t8327: f64, t8406: f64, t8407: f64, t8411: f64, t8413: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t118749 = piecewise3(t115, 0.0_f64, t118655 + t118688 + t118728 + t118746);
    let t118822 = 2.0_f64 * t118749 * t1312 * t569 + 2.0_f64 * t1312 * t1453 * t31653 - 4.0_f64 * t5517 * t651 * t8406 - 4.0_f64 * t13426 * t8407 - 4.0_f64 * t18227 * t8407 + 2.0_f64 * t18245 * t8327 + 2.0_f64 * t2322 * t31654 - 4.0_f64 * t2322 * t31660 - 4.0_f64 * t27123 * t8407 + 4.0_f64 * t27123 * t8411 - 4.0_f64 * t27126 * t8407 + 4.0_f64 * t28219 * t8411 + 4.0_f64 * t28219 * t8413 + 2.0_f64 * t30143 * t8325 + 4.0_f64 * t31382 * t4248 - 4.0_f64 * t31407 * t4248 - 4.0_f64 * t31407 * t7732 + 4.0_f64 * t31459 * t7889 + 2.0_f64 * t31654 * t5523 - 4.0_f64 * t31660 * t4254;
    (t118749, t118822)
}
