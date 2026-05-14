//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 929/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk929<F: Float>(t28149: F, t28193: F, t28233: F, t28281: F, t28325: F, t28372: F, t28413: F, t28458: F, t24191: F, t6752: F, t193: F, t1403: F, t247: F, t28010: F, t28012: F, t28015: F, t28020: F, t28024: F, t28027: F, t28033: F, t28039: F, t28043: F, t28098: F, t28100: F, t5996: F, t6002: F, t6005: F, t6011: F, t6745: F, t6754: F, t6945: F, t719: F) -> (F, F, F, F) {
    let t28461 = t28149 + t28193 + t28233 + t28281 + t28325 + t28372 + t28413 + t28458;
    let t28466 = t24191 * t6752;
    let t28467 = t193 * t28466;
    let t28472 = t28010 * t28012 / 9.0 - t28015 * t6005 / 18.0 - t6002 * t28020 / 18.0 - 2.0 * t28024 + t6002 * t28027 / 9.0 + t6002 * t28033 / 9.0 - t6002 * t28039 / 27.0 + t6002 * t28043 / 9.0 + 2.0 * t28098 - 2.0 * t28100 - t247 * t28461 - t719 * t6945 - t5996 * t6754 / 3.0 - t1403 * t28467 / 3.0 - t6745 * t6011 / 3.0;
    (t28461, t28466, t28467, t28472)
}
