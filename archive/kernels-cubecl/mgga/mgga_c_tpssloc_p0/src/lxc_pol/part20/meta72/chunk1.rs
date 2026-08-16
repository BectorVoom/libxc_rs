//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 523/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk523<F: Float>(t109: F, t1453: F, t656: F, t64: F, t654: F) -> (F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t1454 = t656 * t1453;
    let t1458 = piecewise3::<F>(t110, F::cast_from(0.0_f64), -t654 - t64 * t1454 / F::cast_from(8.0_f64));
    (t1454, t1458)
}
