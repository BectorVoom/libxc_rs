//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1343/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1343(t22194: f64, t833: f64, t16369: f64, t16354: f64, t1102: f64, t11632: f64, t11721: f64, t11723: f64, t1360: f64, t16353: f64, t16601: f64, t22149: f64, t22152: f64, t22156: f64, t22161: f64, t22166: f64, t22170: f64, t22175: f64, t22178: f64, t22181: f64, t22184: f64, t22188: f64, t22191: f64, t22196: f64, t22200: f64, t7028: f64) -> f64 {
    let t22203 = t22194 * t833;
    let t22204 = t16369 * t22203;
    let t22207 = t16354 * t22203;
    let t22210 = -0.32852148333333333333e-3_f64 * t11721 + 0.21901432222222222222e-3_f64 * t11723 + 0.43802864444444444445e-3_f64 * t22149 - 0.295669335e-2_f64 * t1102 * t22152 + 0.65704296666666666667e-3_f64 * t1102 * t22156 - 0.36958666875e-3_f64 * t1102 * t22161 - 0.7391733375e-3_f64 * t1102 * t22166 + 0.1478346675e-2_f64 * t1102 * t22170 - 4.0_f64 * t1360 * t7028 + 0.98556445e-3_f64 * t22175 - t16601 + 0.26281718666666666666e-2_f64 * t11632 * t22178 + 0.26281718666666666666e-2_f64 * t11632 * t22181 - 0.21901432222222222222e-2_f64 * t16353 * t22184 - 0.1478346675e-2_f64 * t1102 * t22188 - 0.87605728888888888887e-3_f64 * t22191 + 0.98556445e-3_f64 * t11632 * t22196 - 0.19711289e-2_f64 * t11632 * t22200 - 0.39422578e-2_f64 * t11632 * t22204 + 0.32852148333333333333e-2_f64 * t16353 * t22207;
    t22210
}
