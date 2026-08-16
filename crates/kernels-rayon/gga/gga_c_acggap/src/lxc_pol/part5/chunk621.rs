//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 621/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk621(t1674: f64, t4051: f64, t2637: f64, t495: f64, t694: f64, t1390: f64, t229: f64, t1378: f64, t276: f64, t40: f64, t1388: f64, t4027: f64, t87: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4052 = t1674 * t4051;
    let t4055 = t694 * t2637 * t495;
    let t4057 = t229 * t1390;
    let t4058 = 8.0_f64 * t4057;
    let t4059 = t1378 * t276;
    let t4060 = t40 * t4059;
    let t4061 = 2.0_f64 * t4060;
    let t4062 = t229 * t1388;
    let t4063 = 8.0_f64 * t4062;
    let t4064 = t4027 * t87;
    (t4052, t4055, t4057, t4058, t4059, t4060, t4061, t4062, t4063, t4064)
}
