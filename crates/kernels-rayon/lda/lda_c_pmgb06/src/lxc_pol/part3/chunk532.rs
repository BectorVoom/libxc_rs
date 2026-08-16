//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 532/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk532(t1979: f64, t1985: f64, t1991: f64, t1997: f64, t1999: f64, t2001: f64, t2004: f64, t2006: f64, t2009: f64, t2014: f64, t2017: f64, t2020: f64, t2034: f64, t2035: f64, t2036: f64, t2037: f64, t2039: f64) -> f64 {
    let t2354 = t1979 - t1985 + t1991 + t1997 + t1999 + t2001 + t2004 + t2006 + t2009 + t2014 - t2017 - t2020 + t2034 + t2035 + t2036 + t2037 + t2039;
    t2354
}
