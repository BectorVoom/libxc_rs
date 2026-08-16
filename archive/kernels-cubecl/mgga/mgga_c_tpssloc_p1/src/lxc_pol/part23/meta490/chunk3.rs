//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1501/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1501<F: Float>(t6330: F, t1315: F, t16101: F, t1799: F, t19781: F, t210: F, t214: F, t221: F, t3733: F, t40025: F, t40401: F, t40422: F, t5195: F, t54663: F, t54725: F, t56535: F, t56539: F, t6347: F, t74726: F, t74747: F, t74756: F, t79921: F, t79984: F) -> (F, F) {
    let t80021 = t6330 * t6330;
    let t80047 = F::cast_from(0.15555555555555555555e-1_f64) * t74747 - t40401 + t40422 + F::cast_from(0.99999999999999999995e-1_f64) * t40025 * t210 * t214 * t80021 - F::cast_from(0.79999999999999999997e-1_f64) * t54663 - F::cast_from(0.13999999999999999999e0_f64) * t74756 + F::cast_from(0.94999999999999999997e-1_f64) * t56535 - F::cast_from(0.31666666666666666666e-1_f64) * t56539 + F::cast_from(0.11111111111111111111e-2_f64) * t54725 - F::cast_from(0.16666666666666666666e-2_f64) * t1315 * t210 * t214 * t79984 + F::cast_from(0.14999999999999999999e-1_f64) * t3733 * t210 * t214 * t79921 + F::cast_from(0.19999999999999999999e-1_f64) * t5195 * t221 * t74726 * t1799 - F::cast_from(0.11999999999999999999e0_f64) * t16101 * t221 * t19781 * t6347;
    (t80021, t80047)
}
