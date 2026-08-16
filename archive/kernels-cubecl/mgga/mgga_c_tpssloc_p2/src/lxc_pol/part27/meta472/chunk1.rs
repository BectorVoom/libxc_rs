//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1836/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1836<F: Float>(t23482: F, t6741: F, t1937: F, t23447: F, t23449: F, t23454: F, t23457: F, t23460: F, t23463: F, t23465: F, t23469: F, t23474: F, t23480: F, t350: F, t378: F, t6747: F) -> (F, F) {
    let t23483 = t23482 * t6741;
    let t23486 = -t23447 - F::cast_from(0.16149102437656156342e-2_f64) * t23449 + F::cast_from(0.72670960969452703541e-2_f64) * t23454 * t1937 - F::cast_from(0.16149102437656156342e-2_f64) * t23457 * t1937 + F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t23460 * t350 - t23463 / F::cast_from(54.0_f64) + t23465 * t378 / F::cast_from(1536.0_f64) - t23469 + F::cast_from(0.20186378047070195428e-3_f64) * t23474 - F::cast_from(0.20186378047070195428e-3_f64) * t23480 - F::cast_from(0.16149102437656156342e-2_f64) * t23483 * t6747;
    (t23483, t23486)
}
