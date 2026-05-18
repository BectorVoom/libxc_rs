//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 971/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk971<F: Float>(t50062: F, t723: F, t1445: F, t3043: F, t39146: F, t45464: F, t45469: F, t45473: F, t45476: F, t45513: F, t45517: F, t45520: F, t45522: F, t45527: F, t45530: F, t45536: F, t45543: F, t45549: F, t45553: F, t45557: F, t47344: F, t47347: F, t50051: F, t813: F, t833: F) -> (F, F) {
    let t50150 = t50062 * t723;
    let t50163 = -F::new(0.46011511144704899612e1) * t813 * t1445 * t50150 + F::new(0.11502877786176224903e2) * t833 * t1445 * t50051 - t45464 + t45469 - t45473 + F::new(0.21450293971110256002e1) * t39146 * t3043 - t45476 - F::new(0.57514388930881124514e0) * t45513 - t45517 + t45520 + F::new(0.76685851907841499352e0) * t45522 - t45527 + t45530 + t45536 - F::new(0.76685851907841499354e0) * t47344 + F::new(0.59584149919750711116e-1) * t47347 - t45543 - t45549 - t45553 - t45557;
    (t50150, t50163)
}
