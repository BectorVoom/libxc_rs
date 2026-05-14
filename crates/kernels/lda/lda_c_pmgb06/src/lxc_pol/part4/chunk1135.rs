//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1135/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1135<F: Float>(t4602: F, t6536: F, t1444: F, t6282: F, t10216: F, t2469: F, t493: F, t1447: F, t6287: F, t6528: F, t2002: F, t5171: F, t16979: F, t16980: F, t16981: F, t16983: F, t16985: F, t16988: F, t16991: F, t16993: F, t16996: F) -> (F, F, F, F, F, F, F) {
    let t16998 = 8.0 / 45.0 * t4602 * t6536;
    let t17000 = 2.0 / 27.0 * t1444 * t6282;
    let t17003 = t493 * t10216 * t2469 / 27.0;
    let t17004 = t1447 * t6287;
    let t17005 = 4.0 / 15.0 * t17004;
    let t17006 = t1447 * t6528;
    let t17007 = 8.0 / 45.0 * t17006;
    let t17009 = 2.0 / 45.0 * t2002 * t5171;
    let t17010 = t16979 + t16980 + t16981 + t16983 + t16985 + t16988 + t16991 + t16993 - t16996 - t16998 + t17000 + t17003 - t17005 + t17007 + t17009;
    (t16998, t17000, t17003, t17005, t17007, t17009, t17010)
}
