//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1116/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1116<F: Float>(t23405: F, t24139: F, t24104: F, t24135: F, t23410: F, t24147: F, t92: F, t165: F, t23884: F, t24148: F, t5769: F, t24143: F, t458: F, t5765: F, t5775: F, t5842: F, t614: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t94260 = t23405 * t24139;
    let t94263 = t23405 * t24104;
    let t94265 = t23405 * t24135;
    let t94267 = t23405 * t23410;
    let t94269 = t24147 * t92;
    let t94285 = t23884 * t165;
    let t94311 = t24148 * t5769;
    let t94327 = t23405 * t24143;
    let t94329 = t5765 * t458;
    let t94330 = t94329 * t5775;
    let t94332 = t5842 * t614;
    (t94260, t94263, t94265, t94267, t94269, t94285, t94311, t94327, t94329, t94330, t94332)
}
