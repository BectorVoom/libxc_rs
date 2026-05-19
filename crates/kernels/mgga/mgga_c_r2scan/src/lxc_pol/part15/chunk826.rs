//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 826/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk826<F: Float>(t6243: F, t7406: F, t1604: F, t2122: F, t2198: F, t5117: F, t5121: F, t6106: F, t6139: F, t7367: F, t7369: F, t7373: F, t7377: F, t7380: F, t7383: F, t7388: F, t7393: F, t7395: F, t7397: F, t7399: F, t7401: F, t7405: F) -> (F, F) {
    let t7407 = t6243 * t7406;
    let t7408 = t1604 * t7407;
    let t7412 = -t7367 - F::cast_from(0.2600466522016280569e0_f64) * t6139 * t7369 + F::cast_from(0.54878743191129263322e-1_f64) * t2122 * t7373 - t7377 - F::cast_from(0.5200933044032561138e0_f64) * t6106 * t7380 + F::cast_from(0.5200933044032561138e0_f64) * t7383 * t2198 - F::cast_from(0.42377972951376424087e0_f64) * t7388 + t7393 + t7395 + t7397 + t7399 + t7401 - t7405 - F::cast_from(0.32927245914677557994e-1_f64) * t7408 + F::cast_from(0.27439371595564631661e-2_f64) * t5117 + F::cast_from(0.29272321618148349056e-1_f64) * t5121;
    (t7407, t7412)
}
