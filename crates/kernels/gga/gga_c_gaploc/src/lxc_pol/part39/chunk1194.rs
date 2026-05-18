//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1194/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1194<F: Float>(t42034: F, t42038: F, t42042: F, t42047: F, t42051: F, t42054: F, t42059: F, t42064: F, t42067: F, t42069: F, t42072: F, t42074: F) -> F {
    let t48044 = F::new(0.35750489951850426669e0) * t42034 + F::new(0.1022478025437886658e1) * t42038 + F::new(0.15337170381568299871e1) * t42042 + t42047 + t42051 - F::new(0.25561950635947166451e1) * t42054 - t42059 - t42064 + t42067 - t42069 + t42072 + F::new(0.71500979903700853338e0) * t42074;
    t48044
}
