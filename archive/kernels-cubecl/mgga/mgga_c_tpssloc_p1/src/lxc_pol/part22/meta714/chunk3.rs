//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2320/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2320<F: Float>(t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t40721: F, t40732: F, t46218: F, t46235: F, t46237: F, t67137: F, t67141: F, t67146: F, t67147: F, t67153: F, t67158: F, t67159: F) -> F {
    let t67451 = t67137 + t46218 + t39463 - t39468 - t40721 - t67141 - t39472 - t39476 - t46235 + t46237 - t40732 - t67146 + t67147 + t67153 + t39483 + t67158 + t67159;
    t67451
}
