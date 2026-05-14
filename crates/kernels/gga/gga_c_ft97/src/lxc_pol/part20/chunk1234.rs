//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1234/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1234<F: Float>(t113346: F, t25162: F, t28752: F, t92: F, t99475: F, t1234: F, t2756: F, t6318: F, t840: F, t28748: F, t113330: F, t113333: F, t113337: F, t113340: F, t113343: F, t99747: F, t99754: F, t99759: F) -> (F, F, F, F) {
    let t113347 = t113346 / 6.0;
    let t113348 = t25162 * t28752;
    let t113349 = t113348 / 9.0;
    let t113350 = t99475 * t92;
    let t113354 = t113350 * t840 * t6318 * t1234 * t2756;
    let t113356 = t25162 * t28748;
    let t113357 = t113356 / 9.0;
    let t113358 = -t113330 - t113333 - 3.0 * t113337 - t99747 + t99754 - t113340 + 2.0 * t113343 - t113347 - t113349 + 15.0 / 16.0 * t113354 - t113357 + t99759;
    (t113348, t113354, t113356, t113358)
}
