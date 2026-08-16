//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 735/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk735(t4684: f64, t557: f64, t11: f64, t1333: f64, t34: f64, t352: f64, t1953: f64, t1948: f64, t954: f64, t213: f64, t558: f64, t174: f64, t3540: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4685 = t557 * t4684;
    let t4686 = t11 * t4685;
    let t4688 = t1333 * t34;
    let t4689 = t4688 * t352;
    let t4690 = t557 * t4689;
    let t4691 = t1953 * t4690;
    let t4693 = t1948 * t954;
    let t4694 = t557 * t4693;
    let t4695 = t11 * t4694;
    let t4697 = t213 * t558;
    let t4699 = t174 * t3540 * t4697;
    (t4685, t4686, t4688, t4689, t4690, t4691, t4693, t4694, t4695, t4697, t4699)
}
