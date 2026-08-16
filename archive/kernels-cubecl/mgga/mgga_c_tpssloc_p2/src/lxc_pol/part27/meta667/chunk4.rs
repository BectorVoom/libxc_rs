//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2347/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2347<F: Float>(t91132: F, t91181: F, t91224: F, t91258: F, t91302: F, t91348: F, t91393: F, t91418: F, t12240: F, t1336: F, t16047: F, t16048: F, t16123: F, t16206: F, t1814: F, t2013: F, t22871: F, t26403: F, t26459: F, t3777: F, t3793: F, t5230: F, t5334: F, t544: F, t553: F, t6987: F, t6990: F, t81216: F, t81218: F, t81230: F, t91065: F, t91074: F, t91077: F, t91078: F, t91081: F, t91091: F) -> (F, F) {
    let t91421 = t91132 + t91181 + t91224 + t91258 + t91302 + t91348 + t91393 + t91418;
    let t91427 = F::cast_from(0.82246703342411321824e-2_f64) * t81216 + F::cast_from(0.38381794893125283518e-1_f64) * t81218 - t1336 * t6987 * t16206 + t91065 - F::cast_from(6.0_f64) * t16047 * t26403 * t16048 + F::cast_from(2.0_f64) * t5334 * t26403 * t12240 + F::cast_from(0.16449340668482264365e-1_f64) * t91074 + t91077 - F::cast_from(0.26044789391763585244e-1_f64) * t91078 + F::cast_from(0.16449340668482264365e-1_f64) * t91081 - F::cast_from(2.0_f64) * t3777 * t26459 - F::cast_from(0.16449340668482264365e-1_f64) * t81230 + F::cast_from(2.0_f64) * t5230 * t6990 + t16123 * t2013 + F::cast_from(0.82246703342411321825e-2_f64) * t91091 + t1814 * t22871 + t544 * t553 * t91421 + F::cast_from(6.0_f64) * t5334 * t26403 * t3793;
    (t91421, t91427)
}
