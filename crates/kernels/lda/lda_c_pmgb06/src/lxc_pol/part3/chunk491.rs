//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 491/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk491<F: Float>(t1995: F, t493: F, t1447: F, t835: F, t1423: F, t806: F, t224: F, t801: F) -> (F, F, F, F) {
    let t1997 = t493 * t1995 / F::new(15.0);
    let t1998 = t1447 * t835;
    let t1999 = F::new(2.0) / F::new(135.0) * t1998;
    let t2000 = t1423 * t806;
    let t2001 = F::new(2.0) / F::new(135.0) * t2000;
    let t2002 = t801 * t224;
    (t1997, t1999, t2001, t2002)
}
