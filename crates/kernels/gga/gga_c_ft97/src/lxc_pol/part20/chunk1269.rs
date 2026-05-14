//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1269/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1269<F: Float>(t113173: F, t113181: F, t113186: F, t113193: F, t113199: F, t113206: F, t113212: F, t113217: F, t114312: F, t114314: F, t114318: F, t114320: F, t113226: F, t113243: F, t113224: F, t113231: F, t113236: F, t113241: F, t99317: F, t99320: F, t99327: F, t99329: F, t99332: F) -> (F, F) {
    let t114324 = -t114312 + 2.0 / 3.0 * t113173 - t114314 - t113181 / 8.0 + t113186 / 6.0 - 4.0 / 27.0 * t113193 + t114318 - 2.0 / 9.0 * t113199 + t114320 - t113206 / 36.0 + t113212 / 9.0 + t113217 / 2.0;
    let t114328 = t113226 / 9.0;
    let t114337 = t113243 / 9.0;
    let t114338 = -2.0 / 9.0 * t113224 + t114328 + 8.0 / 81.0 * t99317 + 2.0 / 27.0 * t99320 - 2.0 / 81.0 * t99327 - t99329 / 27.0 + t99332 / 18.0 + t113231 / 18.0 - t113236 / 6.0 + t113241 / 9.0 + t114337;
    (t114324, t114338)
}
