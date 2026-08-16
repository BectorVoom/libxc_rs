//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1110/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1110<F: Float>(t5120: F, t2475: F, t91: F, t5092: F, t42050: F, t4635: F) -> (F, F, F) {
    let t88231 = t5120 * t5120;
    let t88233 = t91 * t2475 * t88231;
    let t88235 = t5092 * t5092;
    let t88237 = t91 * t42050 * t88235;
    let t88239 = t4635 * t4635;
    (t88233, t88237, t88239)
}
