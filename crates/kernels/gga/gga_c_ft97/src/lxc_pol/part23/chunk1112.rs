//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1112/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1112<F: Float>(t263: F, t9577: F, t1173: F, t668: F, t27882: F, t683: F, t24237: F, t28033: F, t28039: F, t28006: F, t1403: F, t27942: F, t681: F, t27968: F, t5996: F, t6749: F, t98152: F) -> (F, F, F, F, F, F, F, F, F) {
    let t107756 = t263 * t9577;
    let t107765 = t1173 * t668;
    let t107782 = t683 * t27882;
    let t107787 = 2.0 / 27.0 * t24237 * t28033;
    let t107793 = 2.0 / 81.0 * t24237 * t28039;
    let t107806 = t24237 * t28006 / 27.0;
    let t107809 = t1403 * t681 * t27942 / 9.0;
    let t107819 = 2.0 / 9.0 * t5996 * t27968;
    let t107832 = t98152 * t6749 / 27.0;
    (t107756, t107765, t107782, t107787, t107793, t107806, t107809, t107819, t107832)
}
