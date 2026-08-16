//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1063/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1063<F: Float>(t37353: F, t39778: F, t85469: F, t89: F, t1555: F, t9025: F, t356: F, t519: F, t85501: F, t9054: F, t1974: F, t85451: F) -> (F, F, F, F, F) {
    let t86950 = t89 * t37353 * t39778 * t85469;
    let t86954 = t89 * t1555 * t9025 * t85469;
    let t86958 = t89 * t356 * t519 * t85501;
    let t86962 = t89 * t356 * t9054 * t85469;
    let t86966 = t89 * t356 * t1974 * t85451;
    (t86950, t86954, t86958, t86962, t86966)
}
