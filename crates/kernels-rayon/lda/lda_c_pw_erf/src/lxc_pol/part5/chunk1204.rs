//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1204/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1204(t11: f64, t21231: f64, t557: f64, t1953: f64, t21235: f64, t325: f64, t7434: f64, t7440: f64, t352: f64, t7408: f64, t1349: f64, t20907: f64, t3633: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21785 = t11 * t557 * t21231;
    let t21788 = t1953 * t557 * t21235;
    let t21790 = t325 * t7434;
    let t21792 = t325 * t7440;
    let t21794 = t7408 * t352;
    let t21796 = t11 * t1349 * t21794;
    let t21799 = t11 * t3633 * t20907;
    (t21785, t21788, t21790, t21792, t21794, t21796, t21799)
}
