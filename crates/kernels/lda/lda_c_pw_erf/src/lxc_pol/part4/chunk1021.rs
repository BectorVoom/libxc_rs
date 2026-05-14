//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1021/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1021<F: Float>(t1620: F, t838: F, t1931: F, t610: F, t230: F, t4714: F, t211: F, t4567: F, t4575: F, t2127: F, t3455: F, t5065: F, t4703: F, t568: F, t10654: F, t1318: F, t2001: F) -> (F, F, F, F, F, F, F, F) {
    let t13377 = t838 * t1620;
    let t13379 = t1931 * t610;
    let t13381 = t4714 * t230;
    let t13389 = t211 * t4567 * t4575;
    let t13391 = t3455 * t2127;
    let t13397 = t5065 * t2127;
    let t13401 = t4703 * t568;
    let t13419 = t1318 * t10654 * t2001;
    (t13377, t13379, t13381, t13389, t13391, t13397, t13401, t13419)
}
