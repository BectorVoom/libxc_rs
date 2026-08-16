//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1099/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1099(t20: f64, t60: f64, t9108: f64, t94: f64, t102: f64, t9174: f64, t16: f64, t2: f64, t591: f64, t21: f64, t9: f64, t587: f64, t598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32253 = 1.0_f64 / t60 / t20;
    let t35577 = t94 * t9108;
    let t35761 = t102 * t9174;
    let t39030 = 0.7464e2_f64 * t16;
    let t39031 = t2 * t591;
    let t39032 = 0.35904e3_f64 * t39031;
    let t39033 = t9 * t21;
    let t39034 = 1638.0_f64 * t39033;
    let t39035 = t587 * t598;
    (t32253, t35577, t35761, t39030, t39031, t39032, t39033, t39034, t39035)
}
