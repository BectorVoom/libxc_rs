//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2430/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2430(t2979: f64, t3011: f64, t11506: f64, t960: f64, t315: f64, t41224: f64, t2935: f64, t2942: f64, t11408: f64, t941: f64, t2986: f64, t11465: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41751 = t2979 * t3011;
    let t41756 = t960 * t11506;
    let t41759 = t315 * t41224;
    let t41775 = t2935 * t2942;
    let t41779 = t941 * t11408;
    let t41785 = t2979 * t2986;
    let t41788 = t960 * t11465;
    (t41751, t41756, t41759, t41775, t41779, t41785, t41788)
}
