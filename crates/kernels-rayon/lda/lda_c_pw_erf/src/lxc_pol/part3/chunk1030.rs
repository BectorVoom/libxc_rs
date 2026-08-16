//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1030/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1030(t813: f64, t9615: f64, t4505: f64, t668: f64, t4518: f64, t1403: f64, t3974: f64, t5155: f64, t6728: f64, t3667: f64, t573: f64, t3868: f64, t4506: f64, t833: f64) -> (f64, f64, f64, f64, f64) {
    let t12063 = 4.0_f64 / 15.0_f64 * t9615 * t813;
    let t12064 = t4505 * t668;
    let t12065 = t12064 * t4518;
    let t12066 = 32.0_f64 / 45.0_f64 * t12065;
    let t12070 = 16.0_f64 / 15.0_f64 * t3974 * t6728 * t5155 * t1403;
    let t12071 = t573 * t3667;
    let t12075 = 8.0_f64 / 5.0_f64 * t4506 * t12071 * t833 * t3868;
    (t12063, t12064, t12066, t12070, t12075)
}
