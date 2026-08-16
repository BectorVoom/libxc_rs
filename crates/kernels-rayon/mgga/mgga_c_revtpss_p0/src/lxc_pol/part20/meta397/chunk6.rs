//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1471/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1471(t11501: f64, t3014: f64, t2876: f64, t2918: f64, t2924: f64, t11385: f64, t11387: f64, t2875: f64, t11112: f64, t11528: f64, t11116: f64, t11294: f64) -> (f64, f64, f64, f64, f64) {
    let t41832 = t11501 * t3014;
    let t41841 = 36.0_f64 * t2924 * t2876 * t2918;
    let t41845 = 0.3103560775156404018e4_f64 * t11385 * t2875 * t11387 * t2918;
    let t41847 = 24.0_f64 * t11528 * t11112;
    let t41849 = 0.1929837539843104208e3_f64 * t11294 * t11116;
    (t41832, t41841, t41845, t41847, t41849)
}
