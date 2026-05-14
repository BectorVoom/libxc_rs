//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 489/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk489<F: Float>(t1318: F, t2002: F, t504: F, t784: F, t348: F, t1326: F) -> (F, F, F, F) {
    let t2004 = 8.0 / 45.0 * t1318 * t2002;
    let t2005 = t784 * t504;
    let t2006 = t2005 * t348;
    let t2007 = t1326 * t2006;
    (t2004, t2005, t2006, t2007)
}
