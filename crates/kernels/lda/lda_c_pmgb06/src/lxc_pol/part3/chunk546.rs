//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 546/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk546<F: Float>(t1381: F, t2979: F, t493: F, t1485: F, t529: F, t1380: F, t1586: F, t507: F, t1462: F, t517: F, t1477: F, t1604: F, t486: F, t1636: F, t1588: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2980 = t2979 * t1381;
    let t2982 = 2.0 / 15.0 * t493 * t2980;
    let t2983 = t1485 * t529;
    let t2984 = t1380 * t2983;
    let t2986 = t493 * t2984 / 15.0;
    let t2987 = t507 * t1586;
    let t2988 = t1380 * t2987;
    let t2990 = t493 * t2988 / 15.0;
    let t2991 = t1462 * t517;
    let t2992 = t1477 * t529;
    let t2993 = t2991 * t2992;
    let t2995 = t493 * t2993 / 9.0;
    let t2997 = t486 * t1604 / 5.0;
    let t2998 = t486 * t1636;
    let t2999 = 2.0 / 15.0 * t2998;
    let t3001 = t486 * t1588 / 10.0;
    (t2980, t2982, t2983, t2984, t2986, t2987, t2988, t2990, t2991, t2992, t2993, t2995, t2997, t2998, t2999, t3001)
}
