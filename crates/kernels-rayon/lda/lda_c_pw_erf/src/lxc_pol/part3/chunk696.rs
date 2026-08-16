//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 696/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk696(t312: f64, t4288: f64, t1: f64, t1750: f64, t726: f64, t1755: f64, t116: f64, t717: f64, t732: f64, t731: f64, t1184: f64, t1753: f64, t279: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4289 = t4288 * t312;
    let t4291 = t726 * t1750 * t1;
    let t4292 = t4291 * t1755;
    let t4295 = t732 * t717 * t116;
    let t4296 = t731 * t4295;
    let t4299 = t1753 * t1184 * t279;
    (t4289, t4291, t4292, t4295, t4296, t4299)
}
