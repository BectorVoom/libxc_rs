//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2741/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2741(t39490: f64, t39492: f64, t39495: f64, t39498: f64, t39501: f64, t39506: f64, t39508: f64, t39510: f64, t39512: f64, t39515: f64, t682: f64, t701: f64) -> f64 {
    let t39520 = 1.0_f64 * t682 * (-0.21099166666666666667e1_f64 * t39490 + 0.202552e2_f64 * t39492 - 0.75019259259259259258e1_f64 * t39495 + 0.6564185185185185185e1_f64 * t39498 + 0.31003950617283950618e1_f64 * t39501 + 0.68258333333333333335e-1_f64 * t39506 - 0.10921333333333333333e1_f64 * t39508 + 0.12134814814814814815e1_f64 * t39510 + 0.10617962962962962963e1_f64 * t39512 + 0.13388493827160493828e1_f64 * t39515) * t701;
    t39520
}
