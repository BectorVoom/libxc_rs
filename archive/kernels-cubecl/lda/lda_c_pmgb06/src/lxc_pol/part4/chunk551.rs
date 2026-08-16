//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 551/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk551<F: Float>(t1979: F, t1985: F, t1991: F, t1997: F, t1999: F, t2001: F, t2004: F, t2006: F, t2009: F, t2014: F, t2017: F, t2020: F, t2034: F, t2035: F, t2036: F, t2037: F, t2039: F) -> F {
    let t2354 = t1979 - t1985 + t1991 + t1997 + t1999 + t2001 + t2004 + t2006 + t2009 + t2014 - t2017 - t2020 + t2034 + t2035 + t2036 + t2037 + t2039;
    t2354
}
