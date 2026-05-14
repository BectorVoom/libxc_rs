//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 965/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk965<F: Float>(t1554: F, t161: F, t2089: F, t132: F, t2851: F, t823: F, t1512: F, t2015: F, t432: F, t5302: F, t9921: F, t9923: F, t1444: F, t5313: F, t495: F, t5415: F) -> (F, F, F, F, F, F, F, F) {
    let t13087 = t161 * t1554 * t2089;
    let t13088 = t13087 / 45.0;
    let t13090 = t132 * t2851 * t823;
    let t13091 = 4.0 / 405.0 * t13090;
    let t13092 = t1512 * t2015;
    let t13093 = t13092 / 15.0;
    let t13094 = t432 * t5302;
    let t13095 = 2.0 / 15.0 * t13094;
    let t13096 = 4.0 / 135.0 * t9921;
    let t13097 = 2.0 / 45.0 * t9923;
    let t13099 = 2.0 / 15.0 * t1444 * t5313;
    let t13100 = t495 * t5415;
    (t13088, t13091, t13093, t13095, t13096, t13097, t13099, t13100)
}
