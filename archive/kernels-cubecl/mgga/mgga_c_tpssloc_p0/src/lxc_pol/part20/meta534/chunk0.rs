//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2071/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2071<F: Float>(t12204: F, t40409: F, t12214: F, t792: F, t118: F, t12156: F, t794: F, t2229: F, t59: F, t60: F, t535: F, t9538: F) -> (F, F, F, F) {
    let t40410 = t40409 * t12204;
    let t40412 = t792 * t12214;
    let t40415 = t40412 * t118 * t794 * t12156;
    let t40419 = t59 / t60 / t2229;
    let t40422 = F::cast_from(0.26851851851851851851e-2_f64) * t40419 * t535 * t9538;
    (t40410, t40415, t40419, t40422)
}
