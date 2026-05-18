//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 146/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk146<F: Float>(t257: F, t667: F, t109: F, t111: F, t260: F, t271: F, t427: F, t436: F, t437: F, t670: F) -> F {
    let t695 = t257 * t667;
    let t701 = F::new(0.33843946638888888889e-3) * t109 * t427 * t271 - F::new(0.25382959979166666667e-3) * t436 * t437 * t271 - F::new(0.50765919958333333334e-3) * t109 * t111 * t695 - F::new(4.0) * t260 * t670;
    t701
}
