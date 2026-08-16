//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1406/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1406(t2439: f64, t2440: f64, t2829: f64, t10977: f64, t2465: f64, t686: f64, t72: f64, t10513: f64, t10978: f64, t11010: f64, t213: f64, t225: f64, t257: f64, t2765: f64, t2770: f64, t2772: f64, t40876: f64, t41078: f64, t41079: f64, t41085: f64, t41092: f64, t41095: f64, t41098: f64, t41102: f64, t41105: f64, t41115: f64, t41118: f64, t865: f64) -> f64 {
    let t41125 = t2439 * t2440 * t2829;
    let t41129 = t2465 * t10977 * t72 * t686;
    let t41131 = 0.15805078039045227836e2_f64 * t865 * t41078 * t41079 + 0.79025390195226139183e1_f64 * t10513 * t2772 + 0.39512695097613069591e1_f64 * t865 * t2770 * t41085 - 0.13170898365871023197e0_f64 * t41092 - t41095 + 0.18505311230957427423e-1_f64 * t41098 - 0.12142592671231907757e0_f64 * t41102 + 0.78548797528808629095e-3_f64 * t41105 - 0.15805078039045227836e2_f64 * t2765 * t11010 + 0.65854491829355115987e0_f64 * t213 * t40876 * t225 * t257 + 0.78059524315062264152e-1_f64 * t41115 + 0.44178176337912614788e-3_f64 * t41118 - 0.26341796731742046395e1_f64 * t2765 * t10978 - 0.39512695097613069592e1_f64 * t10513 * t2829 + 0.39029762157531132075e-2_f64 * t41125 - 0.39029762157531132076e-1_f64 * t41129;
    t41131
}
