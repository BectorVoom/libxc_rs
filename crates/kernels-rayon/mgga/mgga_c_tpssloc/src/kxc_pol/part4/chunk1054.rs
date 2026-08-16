//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1054/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1054(t10556: f64, t10636: f64, t13563: f64, t13598: f64, t14245: f64, t14246: f64, t17149: f64, t17154: f64, t17159: f64, t17163: f64, t17165: f64, t17169: f64, t17173: f64, t17175: f64, t17180: f64, t17185: f64, t17189: f64) -> f64 {
    let t17488 = -t10636 - 0.79148148148148148147e-2_f64 * t10556 - 0.15829629629629629629e-1_f64 * t13598 + 0.79148148148148148147e-2_f64 * t13563 - t14245 + t14246 + 0.39574074074074074073e-2_f64 * t17149 - 0.19787037037037037037e-1_f64 * t17154 + 0.71233333333333333332e-1_f64 * t17159 - 0.23744444444444444444e-1_f64 * t17163 - 0.11872222222222222222e-1_f64 * t17165 - 0.10685e0_f64 * t17169 + 0.71233333333333333332e-1_f64 * t17173 + 0.5936111111111111111e-2_f64 * t17175 - 0.11872222222222222222e-1_f64 * t17180 + 0.35616666666666666666e-1_f64 * t17185 - 0.17808333333333333333e-1_f64 * t17189;
    t17488
}
