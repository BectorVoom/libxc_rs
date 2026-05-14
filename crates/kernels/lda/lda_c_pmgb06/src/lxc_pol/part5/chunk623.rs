//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 623/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk623<F: Float>(t5: F, t1072: F, t2192: F, t330: F, t332: F, t5953: F, t5958: F, t5961: F, t2386: F, t3548: F, t1219: F, t2389: F, zeta_threshold: F) -> (F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t5965 = piecewise3(t6, 0.0, 8.0 / 27.0 * t5953 * t332 - 8.0 / 9.0 * t2192 * t1072 - 2.0 / 9.0 * t5958 * t332 + 2.0 / 3.0 * t330 * t5961);
    let t5966 = t3548 * t2386;
    let t5971 = t1219 * t2389;
    let t5974 = -t5961;
    (t5965, t5966, t5971, t5974)
}
