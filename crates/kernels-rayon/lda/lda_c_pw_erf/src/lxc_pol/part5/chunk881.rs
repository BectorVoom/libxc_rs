//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 881/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk881(t1084: f64, t156: f64, t2737: f64, t2698: f64, t2704: f64, t2987: f64, t2701: f64, t1055: f64, t474: f64, t39: f64, t55: f64, t59: f64, t87: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8285 = 0.021687161765563047_f64 * t1084 * t156 * t2737;
    let t8286 = t2704 * t2698;
    let t8290 = 38.02486811957057_f64 * t1084 * t156 * t2987;
    let t8291 = t2704 * t2701;
    let t8296 = 1.2842518958703766_f64 * t1084 * t474 * t1055;
    let t8300 = 24.0_f64 * t39 * t55 * t59 * t87;
    (t8285, t8286, t8290, t8291, t8296, t8300)
}
