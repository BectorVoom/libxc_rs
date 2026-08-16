//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1131/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1131(t1696: f64, t1798: f64, t208: f64, t213: f64, t4075: f64, t794: f64, t5374: f64, t588: f64, t97: f64, t3295: f64, t802: f64, t3035: f64) -> (f64, f64, f64, f64, f64) {
    let t13444 = t1798 * t1696 * t208 * t213;
    let t13447 = t794 * t4075 * t208 * t213;
    let t13450 = t5374 * t97 * t588;
    let t13452 = t802 * t3295;
    let t13453 = 2.0_f64 / 15.0_f64 * t13452;
    let t13455 = t802 * t3035 / 5.0_f64;
    (t13444, t13447, t13450, t13453, t13455)
}
