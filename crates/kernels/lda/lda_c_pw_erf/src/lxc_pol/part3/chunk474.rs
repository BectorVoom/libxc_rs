//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 474/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk474<F: Float>(t1325: F, t2007: F, t593: F, t816: F, t1308: F, t571: F, t1319: F, t1949: F, t1485: F, t219: F) -> (F, F, F, F, F, F, F) {
    let t2009 = 8.0 / 45.0 * t1325 * t2007;
    let t2010 = t816 * t593;
    let t2011 = t1308 * t2010;
    let t2013 = 4.0 / 45.0 * t571 * t2011;
    let t2014 = t1319 * t1949;
    let t2016 = 8.0 / 45.0 * t571 * t2014;
    let t2017 = t1485 * t219;
    (t2009, t2010, t2011, t2013, t2014, t2016, t2017)
}
