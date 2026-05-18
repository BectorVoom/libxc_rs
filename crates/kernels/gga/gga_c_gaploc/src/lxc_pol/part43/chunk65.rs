//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 65/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk65<F: Float>(t255: F, t260: F, t109: F, t111: F) -> (F, F, F) {
    let t271 = F::new(1.0) / t255;
    let t275 = t260 * t260;
    let t277 = F::new(0.50765919958333333334e-3) * t109 * t111 * t271 - F::new(2.0) * t275;
    let t278 = F::new(1.0) / t277;
    (t271, t277, t278)
}
