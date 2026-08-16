//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 590/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk590(t1165: f64, t1188: f64, t4298: f64, t407: f64, t4289: f64, t1549: f64, t3409: f64, t1554: f64, t1558: f64, t1016: f64, t524: f64, t1017: f64, t157: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4300 = t1165 * t4298 * t1188;
    let t4304 = t1165 * t4289 * t407;
    let t4308 = 0.40015750243531754508e-2_f64 * t3409 * t1549;
    let t4310 = 0.40015750243531754508e-2_f64 * t3409 * t1554;
    let t4312 = 0.20007875121765877254e-2_f64 * t3409 * t1558;
    let t4313 = t1016 * t524;
    let t4314 = t157 * t1017;
    (t4300, t4304, t4308, t4310, t4312, t4313, t4314)
}
