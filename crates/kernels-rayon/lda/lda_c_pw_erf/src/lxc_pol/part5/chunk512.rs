//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 512/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk512(t1325: f64, t2558: f64, t1334: f64, t2334: f64, t574: f64, t571: f64, t1339: f64, t2325: f64, t522: f64, t519: f64, t1938: f64, t1985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2560 = 8.0_f64 / 15.0_f64 * t1325 * t2558;
    let t2561 = t1334 * t2334;
    let t2562 = t574 * t2561;
    let t2564 = 8.0_f64 / 45.0_f64 * t571 * t2562;
    let t2565 = t1339 * t2325;
    let t2566 = t522 * t2565;
    let t2568 = 8.0_f64 / 45.0_f64 * t519 * t2566;
    let t2569 = 8.0_f64 / 45.0_f64 * t1938;
    let t2570 = 8.0_f64 / 45.0_f64 * t1985;
    (t2560, t2561, t2562, t2564, t2565, t2566, t2568, t2569, t2570)
}
