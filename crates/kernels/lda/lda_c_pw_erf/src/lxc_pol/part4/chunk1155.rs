//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1155/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1155<F: Float>(t12546: F, t12557: F, t12572: F, t12577: F, t12616: F, t12621: F, t12629: F, t12631: F, t12633: F, t12637: F, t12639: F, t12652: F, t12654: F, t12661: F, t12665: F, t12667: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16999 = 32.0 / 45.0 * t12546;
    let t17000 = 32.0 / 135.0 * t12557;
    let t17001 = 64.0 / 1215.0 * t12572;
    let t17002 = 32.0 / 45.0 * t12577;
    let t17003 = 64.0 / 1215.0 * t12616;
    let t17004 = 32.0 / 45.0 * t12621;
    let t17005 = 64.0 / 135.0 * t12629;
    let t17006 = 16.0 / 135.0 * t12631;
    let t17007 = 64.0 / 135.0 * t12633;
    let t17008 = 32.0 / 405.0 * t12637;
    let t17009 = 32.0 / 45.0 * t12639;
    let t17010 = 16.0 / 135.0 * t12652;
    let t17011 = 32.0 / 45.0 * t12654;
    let t17012 = 32.0 / 45.0 * t12661;
    let t17013 = 16.0 / 135.0 * t12665;
    let t17014 = 16.0 / 81.0 * t12667;
    (t16999, t17000, t17001, t17002, t17003, t17004, t17005, t17006, t17007, t17008, t17009, t17010, t17011, t17012, t17013, t17014)
}
