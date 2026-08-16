//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 759/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk759<F: Float>(t10261: F, t10262: F, t27: F, t89: F, t2680: F, t824: F, t2739: F, t193: F, t295: F, t9570: F, t9571: F, t9716: F) -> (F, F, F, F, F, F, F, F) {
    let t10263 = t10261 * t10262;
    let t10265 = t89 * t27 * t10263;
    let t10266 = t2680 * t824;
    let t10267 = t10266 * t2739;
    let t10269 = t89 * t193 * t10267;
    let t10270 = t295 * t9570;
    let t10271 = t10270 * t9571;
    let t10273 = t89 * t9716 * t10271;
    (t10263, t10265, t10266, t10267, t10269, t10270, t10271, t10273)
}
