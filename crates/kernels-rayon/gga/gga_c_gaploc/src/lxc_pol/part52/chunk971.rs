//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 971/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk971(t50062: f64, t723: f64, t1445: f64, t3043: f64, t39146: f64, t45464: f64, t45469: f64, t45473: f64, t45476: f64, t45513: f64, t45517: f64, t45520: f64, t45522: f64, t45527: f64, t45530: f64, t45536: f64, t45543: f64, t45549: f64, t45553: f64, t45557: f64, t47344: f64, t47347: f64, t50051: f64, t813: f64, t833: f64) -> (f64, f64) {
    let t50150 = t50062 * t723;
    let t50163 = -0.46011511144704899612e1_f64 * t813 * t1445 * t50150 + 0.11502877786176224903e2_f64 * t833 * t1445 * t50051 - t45464 + t45469 - t45473 + 0.21450293971110256002e1_f64 * t39146 * t3043 - t45476 - 0.57514388930881124514e0_f64 * t45513 - t45517 + t45520 + 0.76685851907841499352e0_f64 * t45522 - t45527 + t45530 + t45536 - 0.76685851907841499354e0_f64 * t47344 + 0.59584149919750711116e-1_f64 * t47347 - t45543 - t45549 - t45553 - t45557;
    (t50150, t50163)
}
