//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 837/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk837(t5370: f64, t5372: f64, t5376: f64, t5379: f64, t7750: f64, t7752: f64, t7754: f64, t7756: f64, t7758: f64, t7759: f64, t7760: f64, t7761: f64, t7762: f64, t7763: f64, t7765: f64, t7766: f64) -> f64 {
    let t7983 = -t7750 - t7752 - t7754 - t7756 + 4.0_f64 / 3.0_f64 * t5370 - 2.0_f64 / 9.0_f64 * t5372 - t7758 - t7759 - t7760 - t7761 - t7762 - t7763 + t7765 + t7766 + t5376 + 0.36466666666666664_f64 * t5379;
    t7983
}
