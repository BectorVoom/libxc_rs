//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1215/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1215<F: Float>(t1334: F, t2151: F, t571: F, t743: F, t1318: F, t16979: F, t219: F, t35: F, t558: F, t13751: F, t10427: F, t13767: F, t10439: F, t565: F, t6303: F, t1392: F, t1440: F, t2497: F, t3675: F, t519: F) -> (F, F, F, F, F, F, F, F) {
    let t18001 = 32.0 / 45.0 * t571 * t2151 * t1334 * t743;
    let t18006 = 64.0 / 45.0 * t1318 * t16979 * t219 * t35 * t558;
    let t18007 = 64.0 / 135.0 * t13751;
    let t18008 = 32.0 / 405.0 * t10427;
    let t18009 = 16.0 / 45.0 * t13767;
    let t18010 = 8.0 / 135.0 * t10439;
    let t18011 = t565 * t6303;
    let t18012 = 16.0 / 45.0 * t18011;
    let t18017 = 4.0 / 5.0 * t519 * t1440 * t3675 * t2497 * t1392;
    (t18001, t18006, t18007, t18008, t18009, t18010, t18012, t18017)
}
