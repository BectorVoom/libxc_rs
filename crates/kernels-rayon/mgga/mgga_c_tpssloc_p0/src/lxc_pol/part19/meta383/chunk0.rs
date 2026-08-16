//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1431/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1431(t3368: f64, t3375: f64, t11292: f64, t1143: f64, t3324: f64, t3331: f64, t1124: f64, t11419: f64, t11282: f64, t43689: f64, t440: f64, t43713: f64, t43717: f64, t43721: f64, t43725: f64, t43727: f64, t43729: f64, t43734: f64, t43737: f64, t43740: f64, t43743: f64, t43746: f64, t43748: f64, t43750: f64, t43754: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44202 = t3368 * t3375;
    let t44205 = t1143 * t11292;
    let t44211 = t3324 * t3331;
    let t44214 = t1124 * t11419;
    let t44220 = t1143 * t11282;
    let t44223 = t440 * t43689;
    let t44243 = -0.125034e1_f64 * t43713 - 0.13892666666666666667e0_f64 * t43717 + 0.375102e1_f64 * t43721 + 0.83356e0_f64 * t43725 + 0.13772666666666666667e1_f64 * t43727 - 0.41318e1_f64 * t43729 + 0.34431666666666666667e1_f64 * t43734 - 0.123954e2_f64 * t43737 - 0.13772666666666666667e1_f64 * t43740 + 0.185931e2_f64 * t43743 + 0.41318e1_f64 * t43746 - 0.91817777777777777776e0_f64 * t43748 - 0.76514814814814814814e0_f64 * t43750 - 0.104195e0_f64 * t43754;
    (t44202, t44205, t44211, t44214, t44220, t44223, t44243)
}
