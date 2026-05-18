//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 974/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk974<F: Float>(t11317: F, t2701: F, t4397: F, t1085: F, t1798: F, t4: F, t8189: F, t1769: F, t4295: F, t2851: F, t749: F, t1: F, t397: F, t4383: F) -> (F, F, F, F, F, F, F) {
    let t11318 = F::new(0.016265371324172287) * t11317;
    let t11319 = t4397 * t2701;
    let t11320 = F::new(0.4815944609513912) * t11319;
    let t11322 = t1798 * t4 * t1085;
    let t11323 = F::new(0.032530742648344574) * t11322;
    let t11324 = F::new(1.7544670192365612) * t8189;
    let t11325 = t1769 * t4295;
    let t11327 = t2851 * t749;
    let t11328 = F::new(144.0) * t11327;
    let t11330 = t4383 * t1 * t397;
    (t11318, t11320, t11323, t11324, t11325, t11328, t11330)
}
