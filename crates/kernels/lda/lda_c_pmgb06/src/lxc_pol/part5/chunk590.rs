//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 590/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk590<F: Float>(t123: F, t290: F, t317: F, t4001: F, t113: F, t2778: F, t301: F, t1147: F, t701: F, t1321: F, t67: F, t107: F, t1180: F) -> (F, F, F, F, F) {
    let t4005 = F::new(0.9247854820715865) * t123 * t4001 * t290 * t317;
    let t4027 = F::new(0.006715335817467199) * t2778 * t113 * t301;
    let t4030 = t123 * t1147 * t701 * t317;
    let t4042 = F::new(1.0) / t1321 / t67;
    let t4063 = t107 * t1180 * t701;
    (t4005, t4027, t4030, t4042, t4063)
}
