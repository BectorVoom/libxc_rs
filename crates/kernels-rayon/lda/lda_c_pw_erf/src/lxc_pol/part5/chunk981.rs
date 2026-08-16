//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 981/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk981(t133: f64, t14584: f64, t21: f64, t411: f64, t635: f64, t1652: f64, t763: f64, t4: f64, t474: f64, t5607: f64, t2: f64, t39: f64, t756: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14585 = t133 * t14584;
    let t14639 = t21 * t635 * t411;
    let t14640 = t1652 * t763 * t14639;
    let t14641 = 1.9486833333333333_f64 * t14640;
    let t14650 = t4 * t474 * t411;
    let t14651 = t5607 * t14650;
    let t14652 = 3.8973666666666666_f64 * t14651;
    let t14654 = t756 * t2 * t39;
    (t14585, t14639, t14641, t14650, t14652, t14654)
}
