//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 976/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk976(t291: f64, t39: f64, t4092: f64, t2035: f64, t5266: f64, t811: f64, t817: f64, t1200: f64, t820: f64, t283: f64, t1197: f64, t3780: f64, t4125: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19100 = t291 * t39;
    let t19101 = t4092 * t19100;
    let t19103 = t2035 * t5266 * t811;
    let t19106 = t817 * t39;
    let t19107 = t1200 * t19106;
    let t19108 = t5266 * t820;
    let t19109 = t2035 * t19108;
    let t19116 = t811 * t283;
    let t19117 = t19116 * t1197;
    let t19120 = t3780 * t4125;
    (t19100, t19101, t19103, t19106, t19107, t19109, t19117, t19120)
}
