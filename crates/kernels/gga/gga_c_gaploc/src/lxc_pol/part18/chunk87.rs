//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 87/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk87<F: Float>(t260: F, t109: F, t111: F, t271: F) -> (F, F) {
    let t275 = t260 * t260;
    let t277 = F::cast_from(0.50765919958333333334e-3_f64) * t109 * t111 * t271 - F::cast_from(2.0_f64) * t275;
    let t278 = F::cast_from(1.0_f64) / t277;
    (t277, t278)
}
