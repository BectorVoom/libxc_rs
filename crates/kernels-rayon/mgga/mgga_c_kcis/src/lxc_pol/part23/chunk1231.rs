//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1231/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1231(t1380: f64, t16681: f64, t27370: f64, t1464: f64, t2038: f64, t28503: f64, t3954: f64, t16937: f64, t28442: f64, t27369: f64, t1650: f64, t27356: f64, t4012: f64, t5709: f64) -> (f64, f64, f64, f64, f64) {
    let t98064 = t27370 * t16681 * t1380;
    let t98069 = t1464 * t28503 * t2038 * t3954;
    let t98072 = t16937 * t28442;
    let t98074 = 0.20612155671296296296e-4_f64 * t27369 * t98072;
    let t98081 = t5709 * t27356 * t1650 * t4012;
    (t98064, t98069, t98072, t98074, t98081)
}
