//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 801/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk801<F: Float>(t1996: F, t3802: F, t519: F, t1251: F, t806: F, t940: F, t1313: F, t1318: F, t1325: F, t1446: F, t1454: F, t1462: F, t2171: F, t2178: F, t221: F, t3709: F, t5373: F, t5375: F, t5380: F, t5382: F, t5394: F, t5399: F, t5401: F, t5406: F, t5411: F, t5414: F, t5418: F, t571: F, t799: F) -> (F, F, F, F) {
    let t5421 = t3802 * t1996;
    let t5423 = F::new(16.0) / F::new(135.0) * t519 * t5421;
    let t5424 = t806 * t1251;
    let t5425 = t5424 * t940;
    let t5426 = t1313 * t5425;
    let t5429 = t5373 + F::new(4.0) / F::new(15.0) * t571 * t5375 - t5380 - F::new(4.0) / F::new(15.0) * t1325 * t5382 + F::new(4.0) / F::new(45.0) * t2171 * t1454 + F::new(4.0) / F::new(27.0) * t2171 * t1462 + F::new(4.0) / F::new(45.0) * t3709 * t799 + F::new(16.0) / F::new(45.0) * t1446 * t2178 - F::new(4.0) / F::new(15.0) * t1325 * t5394 + t5399 + F::new(4.0) / F::new(15.0) * t5401 * t221 - F::new(16.0) / F::new(45.0) * t1318 * t5406 + t5411 - F::new(16.0) / F::new(45.0) * t1325 * t5414 + F::new(16.0) / F::new(45.0) * t1325 * t5418 - t5423 + F::new(8.0) / F::new(45.0) * t519 * t5426;
    (t5421, t5425, t5426, t5429)
}
