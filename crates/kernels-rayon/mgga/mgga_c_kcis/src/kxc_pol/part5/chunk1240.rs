//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1240/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1240(t19945: f64, t19948: f64, t19951: f64, t19954: f64, t19958: f64, t19961: f64, t19963: f64, t19965: f64, t19967: f64, t19970: f64, t20131: f64, t20134: f64, t20137: f64, t20139: f64, t20143: f64, t20146: f64, t20149: f64, t20152: f64) -> f64 {
    let t20789 = 0.26979166666666666667e-1_f64 * t19945 + 0.29976851851851851851e-2_f64 * t19948 + 0.125e0_f64 * t19951 + 0.1875e0_f64 * t19954 + 0.625e-1_f64 * t19958 - 0.4046875e-1_f64 * t19961 + 0.20234375e-1_f64 * t19963 - 0.20833333333333333333e-1_f64 * t19965 - 0.26979166666666666666e-1_f64 * t19967 - 0.16666666666666666667e0_f64 * t19970 + 0.9375e-1_f64 * t20131 - 0.5e0_f64 * t20134 + 0.375e0_f64 * t20137 - 0.33333333333333333333e0_f64 * t20139 - 0.9375e-1_f64 * t20143 + 0.101171875e-1_f64 * t20146 + 0.25e0_f64 * t20149 - 0.41666666666666666667e-1_f64 * t20152;
    t20789
}
