//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1068/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1068(t20011: f64, t20031: f64, t59: f64, t40: f64, t87: f64, t390: f64, t7376: f64, t339: f64, t7383: f64, t344: f64, t8195: f64, t11335: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20033 = (t20011 + t20031) * t59;
    let t20035 = t40 * t20033 * t87;
    let t20037 = t40 * t7376 * t390;
    let t20038 = t339 * t7383;
    let t20039 = 4.0_f64 * t20038;
    let t20040 = t344 * t7383;
    let t20041 = 4.0_f64 * t20040;
    let t20043 = 24.0_f64 * t8195;
    let t20044 = 10.526802115419367_f64 * t11335;
    (t20033, t20035, t20037, t20039, t20041, t20043, t20044)
}
