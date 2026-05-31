//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 796/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk796<F: Float>(t6230: F, t851: F, t166: F, t161: F, t6307: F, t6309: F, t6311: F, t6313: F, t6315: F, t6317: F, t6319: F, t6321: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7442 = t6230 * t851;
    let t7443 = t166 * t7442;
    let t7445 = t161 * t7443 / F::cast_from(10.0_f64);
    let t7447 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t6307;
    let t7448 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t6309;
    let t7449 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t6311;
    let t7450 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t6313;
    let t7451 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t6315;
    let t7452 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t6317;
    let t7453 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t6319;
    let t7454 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t6321;
    (t7442, t7443, t7445, t7447, t7448, t7449, t7450, t7451, t7452, t7453, t7454)
}
