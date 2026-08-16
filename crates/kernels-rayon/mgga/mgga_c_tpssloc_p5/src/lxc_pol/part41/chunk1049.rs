//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1049/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1049(t16796: f64, t221: f64, t776: f64, t13014: f64, t13020: f64, t13022: f64, t13027: f64, t16784: f64, t16787: f64, t16792: f64, t16794: f64, t4127: f64, t787: f64, t9579: f64, t9583: f64) -> f64 {
    let t16798 = t221 * t16796 * t776;
    let t16803 = -t13014 - 0.24999999999999999999e-2_f64 * t16784 - 0.16666666666666666666e-2_f64 * t787 * t16787 + 0.8333333333333333333e-3_f64 * t16792 + t9579 + 0.38888888888888888887e-2_f64 * t16794 + 0.49999999999999999998e-2_f64 * t4127 * t16798 + 0.77777777777777777775e-2_f64 * t13020 - 0.10555555555555555555e-1_f64 * t13022 + t13027 - t9583;
    t16803
}
