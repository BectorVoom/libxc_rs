//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 758/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk758(t2532: f64, t3416: f64, t2065: f64, t2191: f64, t1466: f64, t1318: f64, t4893: f64, t833: f64, t4892: f64, t5334: f64, t826: f64, t1401: f64, t2466: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6952 = 8.0_f64 / 15.0_f64 * t3416 * t2532;
    let t6953 = t2191 * t2065;
    let t6954 = t1466 * t6953;
    let t6956 = 8.0_f64 / 15.0_f64 * t1318 * t6954;
    let t6957 = t4893 * t833;
    let t6958 = t4892 * t6957;
    let t6960 = 8.0_f64 / 15.0_f64 * t1318 * t6958;
    let t6962 = 8.0_f64 / 45.0_f64 * t5334 * t826;
    let t6963 = t1401 * t2466;
    (t6952, t6953, t6954, t6956, t6957, t6958, t6960, t6962, t6963)
}
