//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 838/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk838(t1457: f64, t41784: f64, t4540: f64, t12918: f64, t1562: f64, t4614: f64, t12766: f64, t597: f64, t40147: f64, t3116: f64, t986: f64) -> (f64, f64, f64, f64, f64) {
    let t41787 = 0.21450293971110256001e1_f64 * t4540 * t1457 * t41784;
    let t41790 = 0.92023022289409799224e1_f64 * t1562 * t4614 * t12918;
    let t41793 = 0.15337170381568299871e2_f64 * t597 * t4614 * t12766;
    let t41800 = 0.11502877786176224903e1_f64 * t40147;
    let t41809 = t986 * t3116;
    (t41787, t41790, t41793, t41800, t41809)
}
