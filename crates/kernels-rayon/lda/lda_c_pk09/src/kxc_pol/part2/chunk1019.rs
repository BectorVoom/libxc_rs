//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1019/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1019(t44: f64, t11007: f64, t11055: f64, t1727: f64, t2727: f64, t427: f64, t11033: f64, zeta_threshold: f64) -> f64 {
    let t45 = t44 <= zeta_threshold;
    let t11058 = piecewise3(t45, t11007, t11055 * t427 + t1727 * t2727);
    let t11059 = t11033 + t11058;
    t11059
}
