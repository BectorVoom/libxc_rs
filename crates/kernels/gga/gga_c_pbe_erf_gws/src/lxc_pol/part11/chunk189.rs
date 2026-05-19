//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 189/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk189<F: Float>(t103: F, t485: F, t395: F, t118: F, t119: F, t120: F, t155: F, t117: F, t4: F) -> (F, F, F, F) {
    let t486 = t485 * t103;
    let t488 = F::cast_from(0.48717083333333333333e0_f64) * t486 * t395;
    let t495 = t118 * t119 * t155 * t120 / F::new(12.0);
    let t496 = t117 * t4;
    (t486, t488, t495, t496)
}
