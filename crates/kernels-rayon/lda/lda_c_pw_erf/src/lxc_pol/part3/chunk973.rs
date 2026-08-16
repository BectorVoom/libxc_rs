//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 973/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk973(t2693: f64, t2695: f64, t887: f64, t1765: f64, t2993: f64, t2998: f64, t8178: f64, t8180: f64, t2710: f64, t4397: f64, t2707: f64, t2698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11305 = t887 * t2693 * t2695;
    let t11307 = t1765 * t2993;
    let t11308 = 3.5089340384731225_f64 * t11307;
    let t11309 = t1765 * t2998;
    let t11310 = 51.94726769812759_f64 * t11309;
    let t11311 = 51.94726769812759_f64 * t8178;
    let t11312 = 3076.1691063023386_f64 * t8180;
    let t11313 = t4397 * t2710;
    let t11314 = 0.021687161765563047_f64 * t11313;
    let t11315 = t4397 * t2707;
    let t11316 = 0.032530742648344574_f64 * t11315;
    let t11317 = t4397 * t2698;
    (t11305, t11308, t11310, t11311, t11312, t11314, t11316, t11317)
}
