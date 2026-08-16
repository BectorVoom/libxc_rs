//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1188/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1188(t2060: f64, t4479: f64, t7815: f64, t142: f64, t2030: f64, t4099: f64, t599: f64, t1078: f64, t2317: f64, t1181: f64, t5249: f64, t7493: f64) -> (f64, f64, f64, f64) {
    let t36262 = t2060 * t7815 * t4479;
    let t36266 = t2030 * t142 * t599 * t4099;
    let t36269 = t2060 * t1078 * t2317;
    let t36273 = t7493 * t1181 * t599 * t5249;
    (t36262, t36266, t36269, t36273)
}
