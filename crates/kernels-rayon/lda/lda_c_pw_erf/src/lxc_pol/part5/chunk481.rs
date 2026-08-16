//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 481/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk481(t2344: f64, t40: f64, t1766: f64, t1773: f64, t1776: f64, t1777: f64, t1779: f64, t2343: f64, t85: f64, t1036: f64, t1045: f64, t1049: f64, t916: f64) -> f64 {
    let t2345 = t40 * t2344;
    let t2346 = 1.169644679491041_f64 * t1766;
    let t2348 = 0.0003662311007350632_f64 * t1773;
    let t2349 = 2.0_f64 * t1776;
    let t2350 = 8.0_f64 * t1777;
    let t2351 = 8.0_f64 * t1779;
    let t2353 = t2343 * t85;
    let t2354 = 0.019751789702565206_f64 * t2353;
    let t2355 = t2345 + t2354 - t2346 - t2350 - t2351 + t2349 - t2348 + t1036 - t1045 - t1049 - t916;
    t2355
}
