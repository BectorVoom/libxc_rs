//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 861/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk861<F: Float>(t12: F, t2386: F, t3922: F, t1079: F, t2389: F, t1072: F, t14: F, t2133: F, t337: F, t5974: F, t257: F, t6053: F, zeta_threshold: F) -> (F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t6054 = t3922 * t2386;
    let t6059 = t1079 * t2389;
    let t6065 = piecewise3::<f64>(t13, F::new(0.0), -F::new(8.0) / F::new(27.0) * t6054 * t337 - F::new(16.0) / F::new(9.0) * t2133 * t1072 + F::new(4.0) / F::new(9.0) * t6059 * t337 + F::new(4.0) / F::new(3.0) * t14 * t5974);
    let t6067 = (t6053 + t6065) * t257;
    (t6054, t6059, t6067)
}
