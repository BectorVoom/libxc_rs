//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1019/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1019(t5401: f64, t568: f64, t1284: f64, t3437: f64, t10436: f64, t548: f64, t2104: f64, t3994: f64, t808: f64, t2114: f64, t4564: f64, t4568: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11936 = t5401 * t568;
    let t11937 = 8.0_f64 / 15.0_f64 * t11936;
    let t11939 = 4.0_f64 / 5.0_f64 * t1284 * t3437;
    let t11940 = t548 * t10436;
    let t11941 = 16.0_f64 / 15.0_f64 * t11940;
    let t11943 = 4.0_f64 / 5.0_f64 * t2104 * t3437;
    let t11945 = 4.0_f64 / 5.0_f64 * t3994 * t808;
    let t11946 = t2114 * t4564;
    let t11947 = 8.0_f64 / 45.0_f64 * t11946;
    let t11948 = t2114 * t4568;
    (t11937, t11939, t11941, t11943, t11945, t11947, t11948)
}
