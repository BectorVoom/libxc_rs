//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 958/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk958(t1401: f64, t1484: f64, t3476: f64, t4500: f64, t3704: f64, t3964: f64, t1621: f64, t1931: f64, t4233: f64, t838: f64, t10162: f64, t2187: f64, t519: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12428 = t1484 * t1401;
    let t12439 = t4500 * t3476;
    let t12475 = t3964 * t3704;
    let t12507 = t1931 * t1621;
    let t12508 = 4.0_f64 * t12507;
    let t12509 = t838 * t4233;
    let t12557 = t519 * t10162 * t2187;
    (t12428, t12439, t12475, t12508, t12509, t12557)
}
