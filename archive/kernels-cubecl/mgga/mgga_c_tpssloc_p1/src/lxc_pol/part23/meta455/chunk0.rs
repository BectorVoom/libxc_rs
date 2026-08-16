//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1313/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1313<F: Float>(t59028: F, t145: F, t185: F, t75929: F, t39658: F, t41258: F, t41262: F, t76024: F, t76025: F, t76026: F, t76027: F, t76030: F, t76031: F, t76034: F) -> (F, F, F) {
    let t76035 = F::cast_from(0.10389515463408878255e3_f64) * t59028;
    let t76037 = t145 * t75929 * t185;
    let t76038 = t76024 + t76025 - t41258 - t41262 - t76026 + t76027 + t76030 - t39658 + t76031 + t76034 - t76035 + t76037;
    (t76035, t76037, t76038)
}
