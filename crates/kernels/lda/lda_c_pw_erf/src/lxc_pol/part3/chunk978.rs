//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 978/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk978<F: Float>(t8249: F, t8251: F, t402: F, t4383: F, t75: F, t8255: F, t1034: F, t1798: F, t40: F, t3153: F, t748: F, t8267: F) -> (F, F, F, F, F, F, F) {
    let t11352 = F::new(1.7544670192365612) * t8249;
    let t11353 = F::new(3.5089340384731225) * t8251;
    let t11355 = t4383 * t75 * t402;
    let t11356 = F::new(1.7544670192365612) * t11355;
    let t11357 = F::new(24.0) * t8255;
    let t11359 = t40 * t1798 * t1034;
    let t11360 = F::new(3.0) * t11359;
    let t11362 = t40 * t748 * t3153;
    let t11363 = F::new(0.06506148529668915) * t8267;
    (t11352, t11353, t11356, t11357, t11360, t11362, t11363)
}
