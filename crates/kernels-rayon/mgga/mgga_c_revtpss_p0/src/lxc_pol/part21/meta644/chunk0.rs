//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2429/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2429(t41306: f64, t315: f64, t41235: f64, t11449: f64, t941: f64, t2941: f64, t2966: f64, t302: f64, t41245: f64, t2969: f64, t11571: f64, t964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41610 = 0.31003950617283950618e1_f64 * t41306;
    let t41658 = t315 * t41235;
    let t41662 = t941 * t11449;
    let t41667 = t302 / t2966 / t2941;
    let t41672 = 0.16979925925925925926e1_f64 * t41245;
    let t41690 = 0.5356037037037037037e1_f64 * t41306;
    let t41738 = t2966 * t2966;
    let t41740 = t302 / t41738;
    let t41741 = t2969 * t2969;
    let t41742 = 1.0_f64 / t41741;
    let t41746 = t11571 * t964;
    (t41610, t41658, t41662, t41667, t41672, t41690, t41740, t41742, t41746)
}
