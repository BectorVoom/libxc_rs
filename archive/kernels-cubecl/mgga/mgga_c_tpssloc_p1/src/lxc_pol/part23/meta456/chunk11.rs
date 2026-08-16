//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1330/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1330<F: Float>(t16606: F, t16625: F, t193: F, t2378: F, t2522: F, t39658: F, t41258: F, t41262: F, t4314: F, t5527: F, t5544: F, t68371: F, t76026: F, t76027: F, t76030: F, t76031: F, t76034: F, t76035: F, t76037: F, t76063: F) -> F {
    let t76556 = F::cast_from(18.0_f64) * t16606 * t2522 * t5544 - F::cast_from(36.0_f64) * t16625 * t4314 * t5527 + F::cast_from(18.0_f64) * t193 * t2378 * t76063 + F::cast_from(36.0_f64) * t193 * t5544 * t68371 - t39658 - t41258 - t41262 - t76026 + t76027 + t76030 + t76031 + t76034 - t76035 + t76037;
    t76556
}
