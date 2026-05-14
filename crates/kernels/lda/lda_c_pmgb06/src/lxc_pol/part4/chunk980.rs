//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 980/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk980<F: Float>(t1423: F, t4609: F, t5203: F, t1995: F, t3223: F, t1981: F, t835: F, t1461: F, t1835: F, t1636: F, t1848: F, t2880: F, t831: F, t4612: F, t5211: F, t1983: F, t485: F, t5210: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13117 = t1423 * t4609;
    let t13119 = t1423 * t5203;
    let t13139 = t3223 * t1995;
    let t13177 = t1981 * t835;
    let t13182 = t1461 * t1835;
    let t13192 = t1848 * t1636;
    let t13194 = t831 * t2880;
    let t13196 = t5211 * t4612;
    let t13199 = t485 * t5210 * t1983;
    (t13117, t13119, t13139, t13177, t13182, t13192, t13194, t13196, t13199)
}
