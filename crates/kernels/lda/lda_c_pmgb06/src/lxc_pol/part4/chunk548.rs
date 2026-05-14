//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 548/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk548<F: Float>(t5: F, t12: F, t113: F, t2414: F, t301: F, t1212: F, t2377: F, t2381: F, t330: F, t1219: F, t2386: F, t2389: F, t336: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t2432 = t2414 * t113 * t301;
    let t2435 = t1212 * t2377;
    let t2437 = t330 * t2381;
    let t2440 = piecewise3(t6, 0.0, -2.0 / 9.0 * t2435 + 2.0 / 3.0 * t2437);
    let t2441 = t1219 * t2386;
    let t2443 = t336 * t2389;
    let t2446 = piecewise3(t13, 0.0, -2.0 / 9.0 * t2441 + 2.0 / 3.0 * t2443);
    let t2448 = t2440 / 2.0 + t2446 / 2.0;
    (t2432, t2435, t2437, t2441, t2443, t2448)
}
