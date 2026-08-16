//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 686/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk686<F: Float>(t313: F, t89: F, t9555: F, t1882: F, t2811: F, t2807: F, t295: F, t9568: F, t2803: F, t8232: F, t842: F, t10397: F) -> (F, F, F, F, F, F, F) {
    let t10749 = F::cast_from(28.0_f64) / F::cast_from(81.0_f64) * t89 * t9555 * t313;
    let t10750 = t1882 * t2811;
    let t10752 = t1882 * t2807;
    let t10758 = t9568 * t295;
    let t10771 = t1882 * t2803;
    let t10773 = t8232 * t842;
    let t10797 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t10397;
    (t10749, t10750, t10752, t10758, t10771, t10773, t10797)
}
