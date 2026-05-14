//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1065/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1065<F: Float>(t1308: F, t7954: F, t108: F, t7763: F, t25528: F, t378: F, t1286: F, t1637: F, t6456: F, t25542: F, t5495: F, t25538: F, t376: F, t100252: F, t100270: F, t100272: F) -> (F, F, F, F, F, F, F, F, F) {
    let t102065 = t7954 * t1308;
    let t102066 = t108 * t7763;
    let t102071 = t378 * t25528;
    let t102076 = t1286 * t1637 * t6456;
    let t102079 = t5495 * t25542 / 9.0;
    let t102082 = t1286 * t376 * t25538 / 9.0;
    let t102103 = 4.0 / 9.0 * t100252;
    let t102109 = t100270 / 3.0;
    let t102110 = 2.0 * t100272;
    (t102065, t102066, t102071, t102076, t102079, t102082, t102103, t102109, t102110)
}
