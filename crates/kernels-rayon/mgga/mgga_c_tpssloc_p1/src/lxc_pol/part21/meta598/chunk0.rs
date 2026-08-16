//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2349/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2349(t20: f64, t60: f64, t1799: f64, t3701: f64, t9108: f64, t94: f64, t102: f64, t9174: f64, t2: f64, t591: f64, t21: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32253 = 1.0_f64 / t60 / t20;
    let t33159 = t3701 * t1799;
    let t35577 = t94 * t9108;
    let t35761 = t102 * t9174;
    let t39031 = t2 * t591;
    let t39033 = t9 * t21;
    (t32253, t33159, t35577, t35761, t39031, t39033)
}
