//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1262/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1262<F: Float>(t124183: F, t124186: F, t124190: F, t124194: F, t124198: F, t124202: F, t124205: F, t124207: F, t124210: F, t124215: F, t124219: F, t1091: F, t24437: F, t24438: F, t27855: F, t108310: F, t3746: F, t6878: F) -> (F, F, F) {
    let t124221 = t124183 / 27.0 + t124186 / 3.0 - t124190 / 9.0 - t124194 / 3.0 - 2.0 / 9.0 * t124198 - 4.0 / 9.0 * t124202 - 2.0 / 9.0 * t124205 + 4.0 / 27.0 * t124207 - t124210 / 18.0 + t124215 / 3.0 + 4.0 / 3.0 * t124219;
    let t124225 = t24437 * t24438 * t27855 * t1091;
    let t124229 = t108310 * t24438 * t6878 * t3746;
    (t124221, t124225, t124229)
}
