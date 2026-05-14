//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1279/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1279<F: Float>(t124183: F, t124207: F, t124210: F, t124186: F, t124190: F, t124194: F, t124198: F, t124202: F, t124205: F, t124215: F, t124219: F, t124235: F, t124250: F, t124225: F, t124229: F, t124232: F, t124240: F, t124244: F, t124247: F, t124253: F, t124257: F, t124262: F, t97377: F) -> (F, F) {
    let t124613 = t124183 / 9.0;
    let t124618 = 4.0 / 9.0 * t124207;
    let t124619 = t124210 / 6.0;
    let t124621 = t124613 + t124186 - t124190 / 3.0 - t124194 - 2.0 / 3.0 * t124198 - 4.0 / 3.0 * t124202 - 2.0 / 3.0 * t124205 + t124618 - t124619 + t124215 + 4.0 * t124219;
    let t124625 = t124235 / 8.0;
    let t124628 = 2.0 / 3.0 * t124250;
    let t124632 = t97377 - t124225 / 6.0 + t124229 / 3.0 - t124232 / 3.0 + t124625 + t124240 / 4.0 + t124244 + 4.0 / 3.0 * t124247 - t124628 + 4.0 * t124253 - 3.0 / 4.0 * t124257 + t124262 / 6.0;
    (t124621, t124632)
}
