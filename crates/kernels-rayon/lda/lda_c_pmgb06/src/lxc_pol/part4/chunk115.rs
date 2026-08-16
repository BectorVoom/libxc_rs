//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 115/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk115(t103: f64, t235: f64, t36: f64, t37: f64) -> (f64, f64, f64) {
    let t265 = 7.05945_f64 * t37 + 1.549425_f64 * t36 + 0.420775_f64 * t235 + 0.1562925_f64 * t103;
    let t268 = 1.0_f64 + 32.16395899738507_f64 / t265;
    let t269 = f64::ln(t268);
    (t265, t268, t269)
}
