//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 411/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk411(t2900: f64, t2901: f64, t848: f64, t2864: f64, t2867: f64, t2869: f64, t2873: f64, t2875: f64, t2877: f64) -> (f64, f64) {
    let t2903 = t2900 * t2901 * t848;
    let t2912 = -0.57538888888888888889e0_f64 * t2864 + 0.11507777777777777778e1_f64 * t2867 + 0.40256666666666666667e0_f64 * t2869 + 0.366775e-1_f64 * t2873 + 0.73355e-1_f64 * t2875 + 0.137975e0_f64 * t2877;
    (t2903, t2912)
}
