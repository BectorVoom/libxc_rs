//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 569/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk569(t40: f64, t4059: f64, t1388: f64, t229: f64, t4027: f64, t87: f64, t483: f64, t803: f64, t2898: f64, t474: f64, t34: f64, t817: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4060 = t40 * t4059;
    let t4061 = 2.0_f64 * t4060;
    let t4062 = t229 * t1388;
    let t4063 = 8.0_f64 * t4062;
    let t4064 = t4027 * t87;
    let t4065 = t40 * t4064;
    let t4068 = t483 * t803;
    let t4069 = t40 * t4068;
    let t4070 = t2898 * t474;
    let t4073 = t817 * t34;
    (t4061, t4063, t4065, t4069, t4070, t4073)
}
