//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 916/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk916(t1445: f64, t41778: f64, t597: f64, t12894: f64, t18658: f64, t3085: f64, t8097: f64, t1457: f64, t4540: f64, t12918: f64, t1562: f64, t4614: f64) -> (f64, f64, f64, f64, f64) {
    let t41781 = 0.11502877786176224903e2_f64 * t597 * t1445 * t41778;
    let t41783 = 0.21450293971110256001e1_f64 * t18658 * t12894;
    let t41784 = t8097 * t3085;
    let t41787 = 0.21450293971110256001e1_f64 * t4540 * t1457 * t41784;
    let t41790 = 0.92023022289409799224e1_f64 * t1562 * t4614 * t12918;
    (t41781, t41783, t41784, t41787, t41790)
}
