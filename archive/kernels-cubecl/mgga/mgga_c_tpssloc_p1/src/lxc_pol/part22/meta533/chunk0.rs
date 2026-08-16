//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2007/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2007<F: Float>(t39300: F, t739: F, t746: F, t1294: F, t3691: F, t9722: F, t2483: F, t268: F, t9778: F) -> (F, F, F, F) {
    let t39302 = t739 * t39300 * t746;
    let t39304 = F::cast_from(0.5848223622634646207e0_f64) * t1294 * t39302;
    let t39305 = t3691 * t9722;
    let t39309 = F::cast_from(0.71233333333333333332e-1_f64) * t268 * t2483 * t9778;
    (t39302, t39304, t39305, t39309)
}
