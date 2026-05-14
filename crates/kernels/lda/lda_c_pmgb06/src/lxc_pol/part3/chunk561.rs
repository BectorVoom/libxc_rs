//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 561/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk561<F: Float>(t1420: F, t1427: F, t1423: F, t1417: F, t1444: F, t1416: F, t1450: F, t493: F, t176: F, t2918: F, t2912: F, t1462: F, t1511: F, t224: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3164 = 2.0 / 15.0 * t1420 * t1427;
    let t3165 = t1423 * t1427;
    let t3166 = 4.0 / 45.0 * t3165;
    let t3168 = 2.0 / 15.0 * t1444 * t1417;
    let t3169 = t1450 * t1416;
    let t3171 = 2.0 / 15.0 * t493 * t3169;
    let t3172 = t176 * t2918;
    let t3173 = t3172 * t2912;
    let t3174 = t1462 * t3173;
    let t3176 = 2.0 / 9.0 * t493 * t3174;
    let t3177 = t1511 * t224;
    (t3164, t3165, t3166, t3168, t3169, t3171, t3173, t3174, t3176, t3177)
}
