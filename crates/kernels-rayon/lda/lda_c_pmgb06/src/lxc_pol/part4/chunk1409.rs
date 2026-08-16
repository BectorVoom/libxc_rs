//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1409/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1409(t1730: f64, t2526: f64, t12804: f64, t16548: f64, t16550: f64, t16555: f64, t16557: f64, t16559: f64, t16560: f64, t16561: f64, t16562: f64, t16566: f64, t16568: f64, t16569: f64, t16573: f64, t16574: f64) -> f64 {
    let t18244 = t2526 * t1730;
    let t18247 = -t16548 + t16550 - t16555 + t16557 - t16559 + t16560 + 0.033245444444444446_f64 * t18244 - t16561 - t16562 + t16566 + t16568 + 16.0_f64 / 81.0_f64 * t12804 - t16569 - t16573 - t16574;
    t18247
}
