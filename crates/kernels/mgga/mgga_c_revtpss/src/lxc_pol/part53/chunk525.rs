//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 525/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk525<F: Float>(t36: F, t4186: F, t70: F, t1470: F, t627: F, t1486: F, t607: F, t1469: F, t2275: F, t606: F, t48: F, t2282: F) -> (F, F, F, F, F, F, F) {
    let t4187 = t36 * t4186;
    let t4188 = t4187 * t70;
    let t4191 = t1470 * t627;
    let t4196 = t607 * t1486;
    let t4201 = t2275 * t1469;
    let t4202 = t4201 * t606;
    let t4205 = t48 * t4186;
    let t4210 = t2282 * t1469;
    (t4187, t4188, t4191, t4196, t4202, t4205, t4210)
}
