//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 813/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk813(t1460: f64, t3245: f64, t10470: f64, t558: f64, t530: f64, t64: f64, t555: f64, t491: f64, t1502: f64, t4188: f64, t1504: f64, t561: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12303 = t3245 * t1460;
    let t12305 = t10470 * t558;
    let t12306 = 0.73697530864197530862e-3_f64 * t12305;
    let t12319 = t64 * t530;
    let t12321 = 1.0_f64 / t555 / t12319;
    let t12322 = t491 * t12321;
    let t12338 = t1502 * t4188;
    let t12343 = t1504 * t1504;
    let t12344 = 1.0_f64 / t12343;
    let t12345 = t561 * t12344;
    (t12303, t12305, t12306, t12321, t12322, t12338, t12343, t12344, t12345)
}
