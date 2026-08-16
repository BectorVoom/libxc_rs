//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1501/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1501(t6330: f64, t1315: f64, t16101: f64, t1799: f64, t19781: f64, t210: f64, t214: f64, t221: f64, t3733: f64, t40025: f64, t40401: f64, t40422: f64, t5195: f64, t54663: f64, t54725: f64, t56535: f64, t56539: f64, t6347: f64, t74726: f64, t74747: f64, t74756: f64, t79921: f64, t79984: f64) -> (f64, f64) {
    let t80021 = t6330 * t6330;
    let t80047 = 0.15555555555555555555e-1_f64 * t74747 - t40401 + t40422 + 0.99999999999999999995e-1_f64 * t40025 * t210 * t214 * t80021 - 0.79999999999999999997e-1_f64 * t54663 - 0.13999999999999999999e0_f64 * t74756 + 0.94999999999999999997e-1_f64 * t56535 - 0.31666666666666666666e-1_f64 * t56539 + 0.11111111111111111111e-2_f64 * t54725 - 0.16666666666666666666e-2_f64 * t1315 * t210 * t214 * t79984 + 0.14999999999999999999e-1_f64 * t3733 * t210 * t214 * t79921 + 0.19999999999999999999e-1_f64 * t5195 * t221 * t74726 * t1799 - 0.11999999999999999999e0_f64 * t16101 * t221 * t19781 * t6347;
    (t80021, t80047)
}
