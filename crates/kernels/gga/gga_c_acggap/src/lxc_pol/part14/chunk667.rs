//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 667/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk667<F: Float>(t1167: F, t7647: F, t1103: F, t1998: F, t1108: F, t1113: F, t1089: F, t368: F, t7554: F, t7553: F, t2037: F, t7309: F, t1966: F, t381: F) -> (F, F, F, F, F, F, F, F) {
    let t7648 = t7647 * t1167;
    let t7650 = t1998 * t1103;
    let t7652 = t1998 * t1108;
    let t7654 = t1998 * t1113;
    let t7670 = t1089 * t368 * t7554;
    let t7671 = t7553 * t7670;
    let t7672 = 0.21437009059034868486e-3 * t7671;
    let t7673 = t7309 * t2037;
    let t7674 = 13.0 / 288.0 * t7673;
    let t7676 = t381 * t1966;
    (t7648, t7650, t7652, t7654, t7670, t7672, t7674, t7676)
}
