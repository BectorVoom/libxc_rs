//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1052/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1052(t1392: f64, t739: f64, t348: f64, t12322: f64, t4488: f64, t108: f64, t267: f64, t510: f64, t4497: f64, t12292: f64, t12294: f64, t12296: f64, t12298: f64, t12301: f64, t12305: f64, t12308: f64, t12310: f64, t12312: f64, t12316: f64, t12320: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12323 = t739 * t1392;
    let t12324 = t12323 * t348;
    let t12327 = 8.0_f64 / 9.0_f64 * t4488 * t12322 * t12324;
    let t12329 = t510 * t108 * t267;
    let t12331 = 16.0_f64 / 15.0_f64 * t12329 * t4497;
    let t12332 = t12292 + t12294 - t12296 + t12298 - t12301 - t12305 + t12308 + t12310 - t12312 - t12316 + t12320 + t12327 + t12331;
    (t12323, t12324, t12327, t12329, t12331, t12332)
}
