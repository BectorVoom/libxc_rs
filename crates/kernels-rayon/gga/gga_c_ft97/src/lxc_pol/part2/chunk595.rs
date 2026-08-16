//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 595/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk595(t193: f64, t4057: f64, t89: f64, t284: f64, t811: f64, t1197: f64, t1196: f64, t816: f64, t820: f64, t1095: f64, t2697: f64, t274: f64, t688: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4059 = t89 * t193 * t4057;
    let t4061 = t811 * t284;
    let t4062 = t4061 * t1197;
    let t4064 = t816 * t1196;
    let t4065 = t4064 * t820;
    let t4068 = t2697 * t1095;
    let t4069 = t274 * t688;
    (t4059, t4061, t4062, t4064, t4065, t4068, t4069)
}
