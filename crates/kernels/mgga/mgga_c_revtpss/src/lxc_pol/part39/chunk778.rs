//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 778/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk778<F: Float>(t36: F, t4186: F, t70: F, t1470: F, t627: F, t1486: F, t607: F, t1469: F, t2275: F, t606: F, t48: F, t2282: F, t60: F, t1474: F, t1480: F, t2290: F, t44: F, t56: F, t614: F, t620: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4187 = t36 * t4186;
    let t4188 = t4187 * t70;
    let t4191 = t1470 * t627;
    let t4196 = t607 * t1486;
    let t4201 = t2275 * t1469;
    let t4202 = t4201 * t606;
    let t4205 = t48 * t4186;
    let t4210 = t2282 * t1469;
    let t4211 = t4210 * t606;
    let t4214 = t60 * t4186;
    let t4217 = -20.0 / 9.0 * t614 * t1474 + 5.0 / 18.0 * t44 * t4202 + 5.0 / 6.0 * t44 * t4205 + 20.0 / 9.0 * t1480 * t620 + 5.0 / 18.0 * t56 * t4211 - 5.0 / 6.0 * t56 * t4214 - t2290;
    (t4187, t4188, t4191, t4196, t4201, t4202, t4205, t4210, t4217)
}
