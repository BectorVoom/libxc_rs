//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2822/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2822(t41245: f64, t41306: f64, t2966: f64, t302: f64, t2969: f64, t2979: f64, t3011: f64, t11506: f64, t960: f64, t315: f64, t41224: f64, t2935: f64, t2942: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41672 = 0.16979925925925925926e1_f64 * t41245;
    let t41690 = 0.5356037037037037037e1_f64 * t41306;
    let t41738 = t2966 * t2966;
    let t41740 = t302 / t41738;
    let t41741 = t2969 * t2969;
    let t41742 = 1.0_f64 / t41741;
    let t41751 = t2979 * t3011;
    let t41756 = t960 * t11506;
    let t41759 = t315 * t41224;
    let t41775 = t2935 * t2942;
    (t41672, t41690, t41740, t41742, t41751, t41756, t41759, t41775)
}
