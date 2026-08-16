//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 510/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk510(t5: f64, t153: f64, t2582: f64, t137: f64, t132: f64, t2377: f64, t2381: f64, t44: f64, t131: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t2583 = t2582 * t153;
    let t2584 = t137 * t2583;
    let t2586 = t132 * t2584 / 30.0_f64;
    let t2590 = piecewise3(t6, 0.0_f64, 2.0_f64 * t5 * t2381 + 2.0_f64 * t2377);
    let t2591 = t2590 * t44;
    let t2592 = t2591 * t131;
    (t2583, t2584, t2586, t2591, t2592)
}
