//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1150/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1150<F: Float>(t3576: F, t822: F, t10371: F, t10403: F, t1446: F, t5360: F, t5397: F, t1318: F, t1466: F, t2156: F, t3563: F, t3770: F, t4763: F) -> (F, F, F, F, F, F, F) {
    let t13464 = t822 * t3576;
    let t13465 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t13464;
    let t13466 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t10371;
    let t13467 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t10403;
    let t13469 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1446 * t5360;
    let t13470 = t1446 * t5397;
    let t13471 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t13470;
    let t13475 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1318 * t1466 * t2156 * t3563;
    let t13477 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t4763 * t3770;
    (t13465, t13466, t13467, t13469, t13471, t13475, t13477)
}
