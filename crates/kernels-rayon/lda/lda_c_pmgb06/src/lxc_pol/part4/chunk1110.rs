//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1110/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1110(t1423: f64, t5268: f64, t5261: f64, t5257: f64, t5242: f64, t5245: f64, t5273: f64, t1447: f64, t5277: f64, t1966: f64, t3031: f64, t5333: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13758 = t1423 * t5268;
    let t13761 = t1423 * t5261;
    let t13763 = t1423 * t5257;
    let t13768 = t1423 * t5242;
    let t13770 = t1423 * t5245;
    let t13775 = t1423 * t5273;
    let t13782 = t1447 * t5277;
    let t13788 = t1966 * t3031;
    let t13807 = t1447 * t5333;
    (t13758, t13761, t13763, t13768, t13770, t13775, t13782, t13788, t13807)
}
