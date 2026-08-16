//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1136/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1136(t13303: f64, t1325: f64, t3859: f64, t4825: f64, t12695: f64, t4830: f64, t1278: f64, t5289: f64, t542: f64, t784: f64, t1997: f64, t3709: f64) -> (f64, f64, f64, f64, f64) {
    let t13304 = 32.0_f64 / 45.0_f64 * t13303;
    let t13306 = t1325 * t3859 * t4825;
    let t13307 = 16.0_f64 / 45.0_f64 * t13306;
    let t13309 = t1325 * t12695 * t4830;
    let t13310 = 16.0_f64 / 9.0_f64 * t13309;
    let t13315 = 8.0_f64 / 5.0_f64 * t1325 * t5289 * t784 * t542 * t1278;
    let t13317 = 4.0_f64 / 15.0_f64 * t3709 * t1997;
    (t13304, t13307, t13310, t13315, t13317)
}
