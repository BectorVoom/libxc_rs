//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 642/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk642<F: Float>(t1089: F, t368: F, t4545: F, t1131: F, t495: F, t1095: F, t879: F, t175: F, t384: F, t1429: F, t997: F, t1418: F, t1347: F, t1165: F, t1532: F, t3084: F) -> (F, F, F, F, F, F, F, F) {
    let t4547 = t1089 * t368 * t4545;
    let t4550 = t495 * t1131;
    let t4552 = t1089 * t1095 * t4550;
    let t4555 = t495 * t879;
    let t4557 = t1089 * t175 * t4555;
    let t4558 = t384 * t4557;
    let t4561 = 0.40015750243531754508e-1 * t997 * t1429;
    let t4563 = 0.16006300097412701803e-1 * t997 * t1418;
    let t4565 = 0.16006300097412701803e-1 * t997 * t1347;
    let t4567 = t1165 * t1532 * t3084;
    (t4547, t4552, t4557, t4558, t4561, t4563, t4565, t4567)
}
