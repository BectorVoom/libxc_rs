//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 793/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk793<F: Float>(t8232: F, t877: F, t2739: F, t840: F, t882: F, t10388: F, t319: F, t2834: F, t681: F, t89: F, t313: F, t9555: F) -> (F, F, F, F, F) {
    let t10735 = t8232 * t877;
    let t10738 = t840 * t882 * t2739;
    let t10741 = t840 * t319 * t10388;
    let t10745 = t89 * t681 * t2834;
    let t10749 = F::new(28.0) / F::new(81.0) * t89 * t9555 * t313;
    (t10735, t10738, t10741, t10745, t10749)
}
