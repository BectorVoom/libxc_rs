//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 643/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk643(t1262: f64, t3531: f64, t286: f64, t2917: f64, t2919: f64, t2922: f64, t2925: f64, t2928: f64, t1207: f64, t1211: f64, t1210: f64, t401: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3532 = t1262 * t1262;
    let t3533 = t3531 * t3532;
    let t3534 = t286 * t3533;
    let t3537 = 0.22831111111111111111e-1_f64 * t2917;
    let t3542 = t3537 + 0.11415555555555555555e-1_f64 * t2919 - 0.11415555555555555555e-1_f64 * t2922 + 0.34246666666666666666e-1_f64 * t2925 - 0.17123333333333333333e-1_f64 * t2928;
    let t3545 = t1207 * t1211;
    let t3548 = t1210 * t401;
    (t3532, t3533, t3534, t3537, t3542, t3545, t3548)
}
