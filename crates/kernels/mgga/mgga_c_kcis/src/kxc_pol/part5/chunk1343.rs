//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1343/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1343<F: Float>(t22194: F, t833: F, t16369: F, t16354: F, t1102: F, t11632: F, t11721: F, t11723: F, t1360: F, t16353: F, t16601: F, t22149: F, t22152: F, t22156: F, t22161: F, t22166: F, t22170: F, t22175: F, t22178: F, t22181: F, t22184: F, t22188: F, t22191: F, t22196: F, t22200: F, t7028: F) -> F {
    let t22203 = t22194 * t833;
    let t22204 = t16369 * t22203;
    let t22207 = t16354 * t22203;
    let t22210 = -F::cast_from(0.32852148333333333333e-3_f64) * t11721 + F::cast_from(0.21901432222222222222e-3_f64) * t11723 + F::cast_from(0.43802864444444444445e-3_f64) * t22149 - F::cast_from(0.295669335e-2_f64) * t1102 * t22152 + F::cast_from(0.65704296666666666667e-3_f64) * t1102 * t22156 - F::cast_from(0.36958666875e-3_f64) * t1102 * t22161 - F::cast_from(0.7391733375e-3_f64) * t1102 * t22166 + F::cast_from(0.1478346675e-2_f64) * t1102 * t22170 - F::new(4.0) * t1360 * t7028 + F::new(0.98556445e-3) * t22175 - t16601 + F::cast_from(0.26281718666666666666e-2_f64) * t11632 * t22178 + F::cast_from(0.26281718666666666666e-2_f64) * t11632 * t22181 - F::cast_from(0.21901432222222222222e-2_f64) * t16353 * t22184 - F::cast_from(0.1478346675e-2_f64) * t1102 * t22188 - F::cast_from(0.87605728888888888887e-3_f64) * t22191 + F::new(0.98556445e-3) * t11632 * t22196 - F::new(0.19711289e-2) * t11632 * t22200 - F::new(0.39422578e-2) * t11632 * t22204 + F::cast_from(0.32852148333333333333e-2_f64) * t16353 * t22207;
    t22210
}
