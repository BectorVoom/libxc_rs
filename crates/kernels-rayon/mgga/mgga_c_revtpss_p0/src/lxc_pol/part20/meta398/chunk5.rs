//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1478/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1478(t300: f64, t41778: f64, t41825: f64, t41853: f64, t41930: f64, t3333: f64, t3335: f64, t11598: f64, t3022: f64, t198: f64, t336: f64, t41577: f64, t41580: f64, t41582: f64, t41585: f64, t41591: f64, t41657: f64, t41841: f64, t41845: f64, t41847: f64, t41849: f64) -> (f64, f64, f64) {
    let t41933 = t300 * (t41778 + t41825 + t41853 + t41930);
    let t41934 = t3333 * t3333;
    let t41936 = t3335 * t3335;
    let t41937 = 1.0_f64 / t41936;
    let t41942 = 0.14035736694323150897e2_f64 * t3022 * t11598;
    let t41943 = -6.0_f64 * t198 * t336 * t41934 * t41937 + t41577 + t41580 + t41582 + t41585 - t41591 + t41657 + t41841 + t41845 - t41847 + t41849 + t41933 - t41942;
    (t41933, t41942, t41943)
}
