//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 908/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk908<F: Float>(t5192: F, t7822: F, t2068: F, t4680: F, t8778: F, t2001: F, t5014: F, t1089: F, t535: F, t7553: F, t7554: F, t7637: F, t8491: F, t1967: F, t8536: F, t4708: F, t7561: F) -> (F, F, F, F, F, F, F) {
    let t34000 = t7822 * t5192;
    let t34003 = t2068 * t4680 * t8778;
    let t34005 = t2001 * t5014;
    let t34009 = t7553 * t1089 * t535 * t7554;
    let t34011 = t7637 * t8491;
    let t34013 = t1967 * t8536;
    let t34014 = 0.64311027177104605458e-2 * t34013;
    let t34015 = t7561 * t4708;
    (t34000, t34003, t34005, t34009, t34011, t34014, t34015)
}
