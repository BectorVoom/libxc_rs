//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1211/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1211<F: Float>(t1472: F, t5322: F, t2163: F, t3727: F, t3416: F, t5317: F, t2171: F, t3896: F, t10502: F, t799: F, t10675: F, t10678: F, t10680: F, t10685: F, t14271: F, t14275: F, t14278: F, t14283: F) -> (F, F, F, F, F, F) {
    let t14285 = F::new(8.0) / F::new(5.0) * t1472 * t5322;
    let t14287 = F::new(4.0) / F::new(5.0) * t3727 * t2163;
    let t14289 = F::new(8.0) / F::new(5.0) * t3416 * t5317;
    let t14291 = F::new(32.0) / F::new(81.0) * t2171 * t3896;
    let t14293 = F::new(4.0) / F::new(45.0) * t10502 * t799;
    let t14296 = t14271 - t14275 - t14278 + t14283 + t14285 + t14287 - t14289 + t14291 + t14293 + t10675 + F::cast_from(0.10821041362364843_f64) * t10678 + F::cast_from(0.6492624817418906_f64) * t10680 + t10685;
    (t14285, t14287, t14289, t14291, t14293, t14296)
}
