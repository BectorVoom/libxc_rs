//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1186/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1186<F: Float>(t5: F, t84202: F, t84231: F, t84258: F, t84287: F, t112: F, t1268: F, t12734: F, t12739: F, t2039: F, t2314: F, t2363: F, t23917: F, t23938: F, t26977: F, t39235: F, t45602: F, t45637: F, t45814: F, t5113: F, t671: F, t7042: F, t7056: F, t84044: F, t84097: F, t84149: F, t9348: F, t9416: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t84290 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t84202 + t84231 + t84258 + t84287);
    let t84291 = t84290 * t112;
    let t84298 = F::cast_from(2.0_f64) * t1268 * t84044 + F::cast_from(12.0_f64) * t12734 * t7056 + F::cast_from(6.0_f64) * t12739 * t7056 + F::cast_from(2.0_f64) * t2039 * t39235 + F::cast_from(6.0_f64) * t2039 * t45602 + F::cast_from(6.0_f64) * t2039 * t45637 + F::cast_from(2.0_f64) * t2039 * t45814 + F::cast_from(6.0_f64) * t2314 * t23917 + F::cast_from(6.0_f64) * t2363 * t23938 + F::cast_from(6.0_f64) * t2363 * t26977 + F::cast_from(6.0_f64) * t23917 * t5113 + F::cast_from(6.0_f64) * t671 * t84097 + F::cast_from(2.0_f64) * t7042 * t9416 + F::cast_from(6.0_f64) * t7056 * t9348 + F::cast_from(6.0_f64) * t84149 + t84291;
    (t84291, t84298)
}
