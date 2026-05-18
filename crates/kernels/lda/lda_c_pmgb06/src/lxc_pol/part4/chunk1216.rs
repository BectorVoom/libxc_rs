//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1216/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1216<F: Float>(t2010: F, t5225: F, t6189: F, t2501: F, t3177: F, t1423: F, t6413: F, t6416: F, t6419: F, t1420: F, t16000: F, t16002: F, t16006: F, t16008: F, t16011: F, t16014: F, t16018: F, t16021: F, t16023: F) -> (F, F, F, F, F, F, F) {
    let t16026 = F::new(16.0) / F::new(45.0) * t2010 * t5225 * t6189;
    let t16028 = F::new(2.0) / F::new(45.0) * t3177 * t2501;
    let t16029 = t1423 * t6413;
    let t16030 = F::new(4.0) / F::new(135.0) * t16029;
    let t16031 = t1423 * t6416;
    let t16032 = F::new(8.0) / F::new(135.0) * t16031;
    let t16033 = t1423 * t6419;
    let t16034 = F::new(4.0) / F::new(81.0) * t16033;
    let t16036 = F::new(2.0) / F::new(45.0) * t1420 * t6413;
    let t16037 = -t16000 + t16002 + t16006 + t16008 + t16011 + t16014 - t16018 + t16021 - t16023 - t16026 - t16028 - t16030 - t16032 + t16034 - t16036;
    (t16026, t16028, t16030, t16032, t16034, t16036, t16037)
}
