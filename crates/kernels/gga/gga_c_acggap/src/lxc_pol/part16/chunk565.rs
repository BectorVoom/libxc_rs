//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 565/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk565<F: Float>(t1418: F, t997: F, t1347: F, t336: F, t506: F, t1008: F, t1429: F, t1487: F, t322: F, t368: F, t398: F, t384: F) -> (F, F, F, F, F, F, F) {
    let t4563 = F::new(0.16006300097412701803e-1) * t997 * t1418;
    let t4565 = F::new(0.16006300097412701803e-1) * t997 * t1347;
    let t4593 = t336 * t506;
    let t4603 = t1008 * t1429;
    let t4623 = t1487 * t322;
    let t4625 = t398 * t368 * t4623;
    let t4627 = F::new(0.85748036236139473944e-3) * t384 * t4625;
    (t4563, t4565, t4593, t4603, t4623, t4625, t4627)
}
