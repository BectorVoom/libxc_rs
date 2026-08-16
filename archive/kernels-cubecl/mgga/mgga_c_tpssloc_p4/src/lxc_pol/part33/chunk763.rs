//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 763/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk763<F: Float>(t1530: F, t25: F, t1408: F, t1877: F, t1915: F, t2522: F, t6670: F, t7476: F, t7541: F, t1539: F, t6690: F) -> (F, F, F) {
    let t7545 = t25 * t1530;
    let t7552 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t7476 + t1877 * t7541 * t25 / F::cast_from(2.0_f64) - t1877 * t6670 * t7545 / F::cast_from(2.0_f64) + t1877 * t1915 * t1408 / F::cast_from(2.0_f64);
    let t7553 = t6690 * t1539;
    (t7545, t7552, t7553)
}
