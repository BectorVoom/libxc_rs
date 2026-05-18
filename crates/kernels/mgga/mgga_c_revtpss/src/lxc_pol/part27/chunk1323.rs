//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1323/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1323<F: Float>(t45551: F, t473: F, t1243: F, t2149: F, t37885: F, t1294: F, t21471: F, t3555: F, t7627: F, t1209: F, t26884: F, t26921: F, t7648: F) -> (F, F, F, F, F, F) {
    let t97377 = t45551 * t473;
    let t97397 = t2149 * t37885 * t1243;
    let t97398 = t21471 * t1294;
    let t97402 = t3555 * t7627;
    let t97419 = t1209 * t26884;
    let t97422 = t7648 * t26921;
    (t97377, t97397, t97398, t97402, t97419, t97422)
}
