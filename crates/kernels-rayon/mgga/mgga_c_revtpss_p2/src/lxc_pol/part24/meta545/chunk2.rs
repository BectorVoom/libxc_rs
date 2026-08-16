//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1614/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1614(t40638: f64, t40654: f64, t50703: f64, t61839: f64, t61877: f64, t61888: f64, t61890: f64, t61892: f64, t61924: f64, t76583: f64, t76587: f64, t76591: f64, t76593: f64, t76596: f64, t76615: f64, t76619: f64, t76645: f64, t76647: f64) -> f64 {
    let t87579 = -0.12196800674228478774e-3_f64 * t61839 - 0.17149607247227894789e-3_f64 * t76583 + 0.68598428988911579156e-3_f64 * t76587 + 0.30492001685571196935e-3_f64 * t76591 - 0.24009450146119052704e0_f64 * t76593 - 0.24009450146119052704e-1_f64 * t76596 + 0.30492001685571196936e-2_f64 * t76615 - 0.34299214494455789577e-3_f64 * t76619 - t40638 + t40654 + 0.6098400337114239387e-4_f64 * t61877 + 0.13011546959266941156e-2_f64 * t50703 + 0.5421477899694558815e-3_f64 * t61888 - 0.13605355082800796532e0_f64 * t61890 - 0.45732285992607719437e-3_f64 * t61892 - 0.34299214494455789577e-3_f64 * t76645 + 0.24009450146119052705e-1_f64 * t76647 - 0.18292914397043087775e-2_f64 * t61924;
    t87579
}
