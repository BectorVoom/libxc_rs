//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1259/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1259<F: Float>(t1154: F, t193: F, t27742: F, t6109: F, t743: F, t109361: F, t110183: F, t110184: F, t110201: F, t110202: F, t124154: F, t124157: F, t124160: F, t124164: F, t124169: F, t124172: F) -> (F, F) {
    let t124177 = t6109 * t193 * t743 * t27742 * t1154;
    let t124180 = t110183 - t110184 + t124154 / 3.0 - 2.0 / 27.0 * t124157 + 5.0 / 81.0 * t124160 - 2.0 / 9.0 * t124164 + t124169 / 6.0 - t124172 / 36.0 + t124177 / 6.0 + t110201 + t110202 + 2.0 / 27.0 * t109361;
    (t124177, t124180)
}
