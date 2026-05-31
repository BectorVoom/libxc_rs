//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1117/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1117<F: Float>(t19614: F, t5083: F, t5084: F, t17621: F, t1911: F, t5068: F, t2389: F, t851: F, t337: F, t5069: F, t5138: F, t5139: F) -> (F, F, F, F, F) {
    let t20420 = t5083 * t5084 * t19614 / F::cast_from(9.0_f64);
    let t20423 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t5068 * t17621 * t1911;
    let t20424 = t2389 * t851;
    let t20425 = t20424 * t337;
    let t20428 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t5068 * t5069 * t20425;
    let t20431 = t5138 * t5139 * t20425 / F::cast_from(9.0_f64);
    (t20420, t20423, t20424, t20428, t20431)
}
