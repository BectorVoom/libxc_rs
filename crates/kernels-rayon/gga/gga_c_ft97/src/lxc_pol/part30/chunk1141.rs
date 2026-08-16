//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1141/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1141(t34265: f64, t6963: f64, t15128: f64, t34230: f64, t25188: f64, t28854: f64, t34089: f64, t10697: f64, t142618: f64, t1466: f64, t193: f64, t28870: f64, t29008: f64, t29033: f64, t29047: f64, t29416: f64, t33966: f64, t34003: f64, t34254: f64, t34312: f64, t36068: f64, t7028: f64, t7581: f64, t7587: f64, t875: f64, t99918: f64) -> (f64, f64, f64, f64) {
    let t153493 = t6963 * t34265;
    let t153507 = t15128 * t34230;
    let t153509 = t25188 * t28854;
    let t153511 = t15128 * t34089;
    let t153520 = -t153493 / 18.0_f64 + t1466 * t193 * t33966 * t29033 + t142618 / 9.0_f64 - t6963 * t34254 / 3.0_f64 - t7581 * t28870 / 3.0_f64 + t34312 * t7028 / 6.0_f64 - t29008 * t34003 / 18.0_f64 + 4.0_f64 * t153507 + 8.0_f64 * t153509 + 8.0_f64 * t153511 - 24.0_f64 * t99918 * t29047 - 24.0_f64 * t10697 * t36068 * t875 - t29416 * t7587 / 3.0_f64;
    (t153507, t153509, t153511, t153520)
}
