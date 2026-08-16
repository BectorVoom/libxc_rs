//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1249/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1249(t122311: f64, t27989: f64, t122314: f64, t125632: f64, t125650: f64, t122336: f64, t122341: f64, t125630: f64, t125637: f64, t125642: f64, t125646: f64, t1955: f64, t2030: f64, t28888: f64) -> f64 {
    let t128652 = t122311 * t27989;
    let t128654 = t122314 * t27989;
    let t128656 = 0.1054086758983270768e-1_f64 * t125632;
    let t128660 = 0.66119071333692697238e-4_f64 * t125650;
    let t128664 = -0.14456046980341999104e-1_f64 * t122336 + t122341 + 0.112937867033921868e-2_f64 * t125630 + 0.28559868832551176308e-1_f64 * t128652 - 0.50779446784275991476e-1_f64 * t128654 + t128656 + 0.56468933516960933999e-3_f64 * t125637 + 0.56468933516960933999e-3_f64 * t125642 - 0.56468933516960933999e-3_f64 * t125646 - t128660 - 0.8673628188205199462e0_f64 * t1955 * t28888 * t2030;
    t128664
}
