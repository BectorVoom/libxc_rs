//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 980/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk980(t8291: f64, t1765: f64, t2987: f64, t1055: f64, t4393: f64, t1051: f64, t1070: f64, t1799: f64, t8303: f64, t8306: f64, t8311: f64, t8313: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11368 = 1.4447833828541736_f64 * t8291;
    let t11369 = t1765 * t2987;
    let t11370 = 1025.3897021007795_f64 * t11369;
    let t11371 = t4393 * t1055;
    let t11372 = 51.94726769812759_f64 * t11371;
    let t11373 = t4393 * t1051;
    let t11374 = 1.7544670192365612_f64 * t11373;
    let t11376 = 96.0_f64 * t1070 * t1799;
    let t11378 = 311.68360618876557_f64 * t8303;
    let t11379 = 0.5848223397455204_f64 * t8306;
    let t11380 = 4.0_f64 * t8311;
    let t11381 = 4.0_f64 * t8313;
    (t11368, t11370, t11372, t11374, t11376, t11378, t11379, t11380, t11381)
}
