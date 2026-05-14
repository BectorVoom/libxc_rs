//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 981/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk981<F: Float>(t19614: F, t5083: F, t5084: F, t17621: F, t1911: F, t5068: F, t2389: F, t851: F, t337: F, t5069: F, t5138: F, t5139: F, t529: F, t6559: F, t9890: F, t2043: F, t2592: F) -> (F, F, F, F, F, F, F) {
    let t20420 = t5083 * t5084 * t19614 / 9.0;
    let t20423 = 2.0 / 5.0 * t5068 * t17621 * t1911;
    let t20424 = t2389 * t851;
    let t20425 = t20424 * t337;
    let t20428 = 2.0 / 15.0 * t5068 * t5069 * t20425;
    let t20431 = t5138 * t5139 * t20425 / 9.0;
    let t20435 = 2.0 / 15.0 * t5068 * t6559 * t20424 * t529;
    let t20436 = 4.0 / 405.0 * t9890;
    let t20438 = t2592 * t2043 / 10.0;
    (t20420, t20423, t20428, t20431, t20435, t20436, t20438)
}
