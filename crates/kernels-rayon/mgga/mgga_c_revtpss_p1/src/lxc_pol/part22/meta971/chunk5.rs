//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3249/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3249(t2674: f64, t40683: f64, t61639: f64, t18441: f64, t9775: f64, t4423: f64, t231: f64, t10698: f64, t10770: f64, t18469: f64, t2430: f64, t2745: f64, t2754: f64, t40425: f64, t50409: f64, t5966: f64, t61623: f64, t61628: f64, t61630: f64, t61632: f64, t825: f64, t827: f64, t828: f64, t851: f64) -> (f64, f64, f64) {
    let t61641 = t2674 * t40683 * t61639;
    let t61645 = t9775 * t18441;
    let t61647 = t4423 * t4423;
    let t61648 = t61647 * t231;
    let t61657 = -0.27104001498285508387e-3_f64 * t61623 + 0.11433071498151929859e-3_f64 * t61628 + 0.24009450146119052704e0_f64 * t61630 - 0.80031500487063509015e-1_f64 * t61632 - 0.25724410870841842183e-1_f64 * t851 * t10698 * t828 * t5966 * t2430 - 0.30492001685571196935e-2_f64 * t61641 - 0.25692334753583138158e-2_f64 * t40425 - 0.40015750243531754508e-2_f64 * t50409 + 0.60976381323476959249e-3_f64 * t61645 - 0.42874018118069736972e-3_f64 * t825 * t827 * t828 * t61648 - 0.42874018118069736972e-2_f64 * t2745 * t10770 * t18469 * t2754;
    (t61647, t61648, t61657)
}
