//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1352/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1352<F: Float>(t1920: F, t21122: F, t21126: F, t21520: F, t21562: F, t21574: F, t23419: F, t28558: F, t2987: F, t4509: F, t6717: F, t7574: F, t88645: F, t99774: F, t99779: F, t99785: F, t99789: F) -> F {
    let t106328 = -F::cast_from(0.30279567070605293142e-3_f64) * t7574 * t28558 + t23419 * t21574 / F::cast_from(768.0_f64) + t6717 * t21562 / F::cast_from(48.0_f64) - t88645 / F::cast_from(2304.0_f64) - F::cast_from(0.30279567070605293142e-3_f64) * t99774 + t1920 * t4509 * t21122 / F::cast_from(72.0_f64) - t23419 * t21520 / F::cast_from(384.0_f64) + F::cast_from(0.30279567070605293142e-3_f64) * t99779 + t99785 / F::cast_from(288.0_f64) + t99789 / F::cast_from(216.0_f64) - t1920 * t2987 * t21126 / F::cast_from(48.0_f64);
    t106328
}
