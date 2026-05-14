//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1046/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1046<F: Float>(t25135: F, t317: F, t28489: F, t29451: F, t263: F, t9577: F, t1173: F, t668: F, t27882: F, t683: F, t24237: F, t28033: F, t28039: F, t41416: F, t6930: F, t1168: F, t24564: F, t2568: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t99993 = t25135 * t317;
    let t107750 = 2.0 * t28489;
    let t107751 = 2.0 * t29451;
    let t107756 = t263 * t9577;
    let t107765 = t1173 * t668;
    let t107782 = t683 * t27882;
    let t107787 = 2.0 / 27.0 * t24237 * t28033;
    let t107793 = 2.0 / 81.0 * t24237 * t28039;
    let t107794 = t41416 * t6930;
    let t107797 = t2568 * t24564 * t1168;
    (t99993, t107750, t107751, t107756, t107765, t107782, t107787, t107793, t107794, t107797)
}
