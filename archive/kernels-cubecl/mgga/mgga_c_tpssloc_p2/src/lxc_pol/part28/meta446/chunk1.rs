//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1632/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1632<F: Float>(t109: F, t22468: F, t22471: F, t22474: F, t22476: F) -> (F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t23912 = F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t22468;
    let t23917 = piecewise3::<F>(t110, F::cast_from(0.0_f64), t23912 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t22471 + t22474 / F::cast_from(2.0_f64) - t22476 / F::cast_from(4.0_f64));
    (t23912, t23917)
}
