//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1010/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1010<F: Float>(t12801: F, t7130: F, t12748: F, t7527: F, t41702: F, t41769: F, t41772: F, t48213: F, t48215: F, t48219: F, t48223: F, t48225: F, t48227: F, t3392: F, t3479: F, t12480: F, t1820: F, t1821: F, t995: F) -> (F, F, F, F, F, F, F, F) {
    let t48229 = 32.0 / 9.0 * t7130 * t12801;
    let t48231 = 64.0 / 15.0 * t7527 * t12748;
    let t48232 = 64.0 / 27.0 * t41702;
    let t48233 = 32.0 / 27.0 * t41769;
    let t48234 = 64.0 / 27.0 * t41772;
    let t48235 = -t48213 + t48215 + t48219 + t48223 - t48225 + t48227 + t48229 - t48231 + t48232 + t48233 + t48234;
    let t48261 = 8.0 / 5.0 * t3479 * t3392;
    let t48265 = 32.0 / 45.0 * t1820 * t1821 * t12480 * t995;
    (t48229, t48231, t48232, t48233, t48234, t48235, t48261, t48265)
}
