//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1140/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1140<F: Float>(t41911: F, t41912: F, t88252: F, t89: F, t27: F, t676: F, t88939: F, t2372: F, t88289: F, t41848: F, t88294: F, t666: F, t669: F, t86571: F) -> (F, F, F, F, F) {
    let t89047 = t89 * t41911 * t41912 * t88252;
    let t89051 = t89 * t27 * t676 * t88939;
    let t89054 = t89 * t27 * t2372 * t88289;
    let t89058 = t89 * t27 * t41848 * t88294;
    let t89062 = t89 * t666 * t669 * t86571;
    (t89047, t89051, t89054, t89058, t89062)
}
