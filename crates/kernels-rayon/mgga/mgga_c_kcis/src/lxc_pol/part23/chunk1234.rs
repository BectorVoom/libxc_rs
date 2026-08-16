//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1234/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1234(t1593: f64, t28352: f64, t498: f64, t27369: f64, t27370: f64, t4007: f64, t5885: f64, t12234: f64, t1938: f64, t3715: f64, t28419: f64, t52649: f64, t7908: f64) -> (f64, f64, f64, f64, f64) {
    let t98137 = t1593 * t498 * t28352;
    let t98138 = t27369 * t98137;
    let t98141 = t27370 * t5885 * t4007;
    let t98144 = t12234 * t1938;
    let t98146 = t27370 * t98144 * t3715;
    let t98150 = t7908 * t52649 * t28419;
    (t98137, t98138, t98141, t98146, t98150)
}
