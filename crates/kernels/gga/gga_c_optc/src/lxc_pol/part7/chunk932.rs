//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 932/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk932<F: Float>(t241: F, t8805: F, t8899: F, t8869: F, t8691: F, t8695: F, t8703: F, t8705: F, t8707: F, t8742: F, t8745: F, t8747: F, t8753: F, t8898: F) -> (F, F, F) {
    let t8901 = t241 * (t8805 + t8899);
    let t8903 = F::new(0.19751789702565206229e-1) * t241 * t8869;
    let t8904 = t8691 - t8695 - t8703 - t8705 - t8707 - t8742 - t8745 + t8747 + t8753 + t8901 + t8903 - t8898;
    (t8901, t8903, t8904)
}
