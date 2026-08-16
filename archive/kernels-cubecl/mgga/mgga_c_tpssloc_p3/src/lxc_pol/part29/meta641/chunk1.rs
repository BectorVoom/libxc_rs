//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2110/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2110<F: Float>(t22690: F, t234: F, t7496: F, t776: F, t81792: F, t23109: F, t23110: F, t232: F, t236: F, t4233: F, t25132: F, t81876: F) -> (F, F, F, F) {
    let t87202 = t22690 * t234;
    let t87205 = t81792 * t87202 * t7496 * t776;
    let t87206 = F::cast_from(0.28260929265898273598e-2_f64) * t87205;
    let t87211 = t23109 * t23110 * t236 * t4233 * t232;
    let t87212 = F::cast_from(0.6728792682356731809e-4_f64) * t87211;
    let t87213 = t81876 * t25132;
    (t87202, t87206, t87212, t87213)
}
