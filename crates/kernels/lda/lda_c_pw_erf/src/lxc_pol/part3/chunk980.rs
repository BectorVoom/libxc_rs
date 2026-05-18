//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 980/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk980<F: Float>(t8291: F, t1765: F, t2987: F, t1055: F, t4393: F, t1051: F, t1070: F, t1799: F, t8303: F, t8306: F, t8311: F, t8313: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11368 = F::new(1.4447833828541736) * t8291;
    let t11369 = t1765 * t2987;
    let t11370 = F::new(1025.3897021007795) * t11369;
    let t11371 = t4393 * t1055;
    let t11372 = F::new(51.94726769812759) * t11371;
    let t11373 = t4393 * t1051;
    let t11374 = F::new(1.7544670192365612) * t11373;
    let t11376 = F::new(96.0) * t1070 * t1799;
    let t11378 = F::new(311.68360618876557) * t8303;
    let t11379 = F::new(0.5848223397455204) * t8306;
    let t11380 = F::new(4.0) * t8311;
    let t11381 = F::new(4.0) * t8313;
    (t11368, t11370, t11372, t11374, t11376, t11378, t11379, t11380, t11381)
}
