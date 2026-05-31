//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2258/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2258<F: Float>(t2251: F, t4173: F, t10301: F, t28126: F, t2247: F, t28076: F, t38: F, t28104: F, t644: F, t77: F, t1928: F, t25102: F, t25110: F, t25117: F, t25157: F, t28138: F, t28141: F, t28147: F, t6960: F, t6974: F, t6978: F, t7716: F, t7720: F, t92684: F, t92687: F) -> F {
    let t101376 = t4173 * t2251;
    let t101385 = t10301 * t28126;
    let t101391 = t2247 * t38 * t28076;
    let t101399 = t77 * t28104 * t644;
    let t101402 = t25117 * t7716 / F::cast_from(3.0_f64) + t25117 * t7720 / F::cast_from(3.0_f64) + t101376 * t1928 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28141 * t6974 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t28138 * t25110 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28141 * t6978 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t101385 * t6960 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t25102 * t7716 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t101391 * t6960 - F::cast_from(10.0_f64) * t92684 * t28147 - F::cast_from(10.0_f64) * t92687 * t28147 - F::cast_from(10.0_f64) * t25157 * t101399;
    t101402
}
