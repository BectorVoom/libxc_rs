//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 559/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk559<F: Float>(t79: F, t1742: F, t4417: F, t420: F, t419: F, t423: F, t4431: F, t1731: F, t3086: F, t4481: F, t409: F, t64: F, t1599: F, t1624: F, t372: F, t4442: F, t4446: F, t4450: F, t4468: F, t4471: F, t4476: F) -> (F, F, F, F, F, F, F, F, F) {
    let t80 = F::new(0.1e-59) < t79;
    let t4483 = t1742 * t4417;
    let t4484 = t420 * t4483;
    let t4485 = t419 * t4484;
    let t4487 = t423 * t4431;
    let t4488 = t420 * t4487;
    let t4489 = t419 * t4488;
    let t4491 = -t1731 + F::new(0.42562405586419753086e-2) * t3086 + F::new(0.85124811172839506173e-2) * t4481 - F::new(0.12768721675925925926e-1) * t4485 + F::new(0.6384360837962962963e-2) * t4489;
    let t4492 = t409 * t4491;
    let t4493 = t64 * t4492;
    let t4495 = piecewise3::<f64>(t80, F::new(0.67598802253579164263e-4) * t4442 * t1599 + F::new(0.23254900946437792e-1) * t1624 * t4446 + F::new(0.23254900946437792e-2) * t372 * t4450 - F::new(0.11627450473218896e-1) * t372 * t4468 + F::new(0.19365723406274399941e-3) * t372 * t4471 + F::new(2.0) * t4476 - t4493, F::new(0.0));
    (t4483, t4484, t4485, t4487, t4488, t4489, t4491, t4493, t4495)
}
