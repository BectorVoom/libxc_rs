//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2252/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2252(t91132: f64, t91181: f64, t91224: f64, t91258: f64, t91302: f64, t91348: f64, t91393: f64, t91418: f64, t12240: f64, t1336: f64, t16047: f64, t16048: f64, t16123: f64, t16206: f64, t1814: f64, t2013: f64, t22871: f64, t26403: f64, t26459: f64, t3777: f64, t3793: f64, t5230: f64, t5334: f64, t544: f64, t553: f64, t6987: f64, t6990: f64, t81216: f64, t81218: f64, t81230: f64, t91065: f64, t91074: f64, t91077: f64, t91078: f64, t91081: f64, t91091: f64) -> (f64, f64) {
    let t91421 = t91132 + t91181 + t91224 + t91258 + t91302 + t91348 + t91393 + t91418;
    let t91427 = 0.82246703342411321824e-2_f64 * t81216 + 0.38381794893125283518e-1_f64 * t81218 - t1336 * t6987 * t16206 + t91065 - 6.0_f64 * t16047 * t26403 * t16048 + 2.0_f64 * t5334 * t26403 * t12240 + 0.16449340668482264365e-1_f64 * t91074 + t91077 - 0.26044789391763585244e-1_f64 * t91078 + 0.16449340668482264365e-1_f64 * t91081 - 2.0_f64 * t3777 * t26459 - 0.16449340668482264365e-1_f64 * t81230 + 2.0_f64 * t5230 * t6990 + t16123 * t2013 + 0.82246703342411321825e-2_f64 * t91091 + t1814 * t22871 + t544 * t553 * t91421 + 6.0_f64 * t5334 * t26403 * t3793;
    (t91421, t91427)
}
