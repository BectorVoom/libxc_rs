//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1150/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1150<F: Float>(t2979: F, t493: F, t6755: F, t1380: F, t529: F, t6827: F, t1586: F, t2549: F, t10099: F, t10109: F, t13291: F, t13294: F, t13332: F, t13337: F, t17049: F, t17052: F, t17054: F, t17057: F, t17059: F, t17061: F, t17064: F, t17066: F, t17072: F, t17075: F) -> (F, F, F, F, F, F, F, F) {
    let t17296 = 2.0 / 45.0 * t493 * t2979 * t6755;
    let t17300 = 2.0 / 45.0 * t493 * t1380 * t6827 * t529;
    let t17304 = t493 * t1380 * t2549 * t1586 / 45.0;
    let t17305 = 2.0 / 243.0 * t10099;
    let t17306 = 2.0 / 405.0 * t10109;
    let t17307 = 4.0 / 135.0 * t13291;
    let t17308 = 4.0 / 135.0 * t13294;
    let t17321 = 0.010075555555555556 * t17049 - 0.030226666666666666 * t17052 - 0.0012594444444444445 * t17054 - 0.005037777777777778 * t17057 - 0.0016792592592592592 * t17059 + 0.000559753086419753 * t17061 + 0.015113333333333333 * t17064 + 0.0008396296296296296 * t17066 - 0.007556666666666666 * t13332 + 0.0033585185185185185 * t13337 - 0.09068 * t17072 + 0.06045333333333333 * t17075;
    (t17296, t17300, t17304, t17305, t17306, t17307, t17308, t17321)
}
