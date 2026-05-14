//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1033/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1033<F: Float>(t93458: F, t1637: F, t5696: F, t89: F, t1316: F, t1771: F, t5677: F, t458: F, t5664: F) -> (F, F, F, F, F, F, F) {
    let t93459 = 4.0 / 9.0 * t93458;
    let t93474 = t89 * t1637 * t5696;
    let t93475 = 8.0 / 9.0 * t93474;
    let t93503 = t1316 * t1771;
    let t93504 = t93503 * t5677;
    let t93505 = 2.0 / 27.0 * t93504;
    let t93506 = t5664 * t458;
    (t93459, t93474, t93475, t93503, t93504, t93505, t93506)
}
