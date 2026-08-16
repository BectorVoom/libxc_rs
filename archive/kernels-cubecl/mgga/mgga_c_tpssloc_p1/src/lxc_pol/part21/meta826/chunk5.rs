//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2917/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2917<F: Float>(t300: F, t59928: F, t59982: F, t60030: F, t60346: F, t60401: F, t60711: F, t60763: F, t60806: F, t17955: F, t2940: F, t17930: F) -> (F, F, F) {
    let t60810 = t300 * (t59928 + t59982 + t60030 + t60346 + t60401 + t60711 + t60763 + t60806);
    let t60812 = F::cast_from(0.34631718211362927518e2_f64) * t2940 * t17955;
    let t60814 = F::cast_from(0.69263436422725855036e2_f64) * t2940 * t17930;
    (t60810, t60812, t60814)
}
