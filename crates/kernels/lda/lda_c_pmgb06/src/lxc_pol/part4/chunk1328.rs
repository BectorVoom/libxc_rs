//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1328/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1328<F: Float>(t2386: F, t337: F, t529: F, t12529: F, t12530: F, t12535: F, t13300: F, t17070: F, t3247: F, t5065: F, t6678: F, t12537: F, t5139: F) -> (F, F, F, F, F) {
    let t17457 = t2386 * t337 * t529;
    let t17460 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t12529 * t12530 * t17457;
    let t17465 = F::cast_from(64.0_f64) / F::cast_from(81.0_f64) * t5065 * t12535 * t3247 * t13300 * t17070;
    let t17466 = t6678 * t529;
    let t17469 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t12537 * t5139 * t17466;
    (t17457, t17460, t17465, t17466, t17469)
}
