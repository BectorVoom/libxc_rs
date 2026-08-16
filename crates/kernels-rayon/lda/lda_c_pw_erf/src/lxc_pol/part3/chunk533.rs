//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 533/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk533(t2215: f64, t2315: f64, t309: f64, t310: f64, t311: f64, t305: f64, t296: f64, t343: f64, t1051: f64, t156: f64, t1084: f64, t1055: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2316 = t2215 + t2315;
    let t2693 = 1.0_f64 / t311 / t310 / t309;
    let t2694 = t305 * t2693;
    let t2695 = t343 * t296;
    let t2696 = t2694 * t2695;
    let t2698 = t156 * t1051;
    let t2699 = t1084 * t2698;
    let t2700 = 0.016265371324172287_f64 * t2699;
    let t2701 = t156 * t1055;
    (t2316, t2693, t2694, t2695, t2696, t2698, t2699, t2700, t2701)
}
