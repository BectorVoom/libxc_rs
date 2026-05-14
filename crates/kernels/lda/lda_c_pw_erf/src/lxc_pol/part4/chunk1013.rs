//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1013/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1013<F: Float>(t10162: F, t1325: F, t2182: F, t518: F, t5400: F, t10011: F, t4480: F, t108: F, t2113: F, t267: F, t5170: F, t565: F, t1498: F, t2123: F, t2010: F, t571: F, t9313: F) -> (F, F, F, F, F, F, F) {
    let t12998 = t1325 * t10162 * t2182;
    let t13014 = t5400 * t518;
    let t13032 = t10011 * t4480;
    let t13035 = t2113 * t108 * t267;
    let t13041 = t565 * t5170;
    let t13043 = t1498 * t2123;
    let t13048 = t571 * t9313 * t2010;
    (t12998, t13014, t13032, t13035, t13041, t13043, t13048)
}
