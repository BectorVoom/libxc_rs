//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 798/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk798(t4106: f64, t531: f64, t3393: f64, t4227: f64, t1520: f64, t752: f64, t1466: f64, t4243: f64, t11824: f64, t569: f64, t3733: f64, t4291: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12417 = t4106 * t531;
    let t12427 = t3393 * t4227;
    let t12431 = t752 * t1520;
    let t12504 = t4243 * t1466;
    let t12505 = t12504 * sigma2;
    let t12520 = t569 * t11824;
    let t12530 = t3733 * t4291;
    (t12417, t12427, t12431, t12504, t12505, t12520, t12530)
}
