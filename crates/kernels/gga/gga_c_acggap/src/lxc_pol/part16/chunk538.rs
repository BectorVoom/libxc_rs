//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 538/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk538<F: Float>(t1095: F, t398: F, t4521: F, t384: F, t1441: F, t997: F, t1451: F, t495: F, t879: F, t1089: F, t175: F, t1429: F, t1418: F, t1347: F, t336: F, t506: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4523 = t398 * t1095 * t4521;
    let t4524 = t384 * t4523;
    let t4532 = 0.16006300097412701803e-1 * t997 * t1441;
    let t4538 = t997 * t1451;
    let t4555 = t495 * t879;
    let t4557 = t1089 * t175 * t4555;
    let t4558 = t384 * t4557;
    let t4561 = 0.40015750243531754508e-1 * t997 * t1429;
    let t4563 = 0.16006300097412701803e-1 * t997 * t1418;
    let t4565 = 0.16006300097412701803e-1 * t997 * t1347;
    let t4593 = t336 * t506;
    (t4523, t4524, t4532, t4538, t4555, t4557, t4558, t4561, t4563, t4565, t4593)
}
