//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 877/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk877<F: Float>(t8291: F, t1765: F, t2987: F, t1055: F, t4393: F, t1051: F, t1070: F, t1799: F, t8303: F, t8306: F, t8311: F, t8313: F, t8357: F, t8375: F, t8296: F, t8300: F, t8301: F, t8309: F, t8356: F, t8368: F, t8373: F, t8382: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11368 = 1.4447833828541736 * t8291;
    let t11369 = t1765 * t2987;
    let t11370 = 1025.3897021007795 * t11369;
    let t11371 = t4393 * t1055;
    let t11372 = 51.94726769812759 * t11371;
    let t11373 = t4393 * t1051;
    let t11374 = 1.7544670192365612 * t11373;
    let t11376 = 96.0 * t1070 * t1799;
    let t11378 = 311.68360618876557 * t8303;
    let t11379 = 0.5848223397455204 * t8306;
    let t11380 = 4.0 * t8311;
    let t11381 = 4.0 * t8313;
    let t11382 = 8.0 * t8357;
    let t11383 = 0.0001831155503675316 * t8375;
    let t11384 = t11368 - t8296 - t11370 - t11372 - t11374 - t8300 - t11376 - 5.476843845342223 * t8301 + t11378 - t11379 + t8309 - t11380 - t11381 - t8356 + t11382 - t8368 - t8373 - t11383 - t8382;
    (t11368, t11370, t11372, t11374, t11376, t11378, t11379, t11380, t11381, t11382, t11383, t11384)
}
