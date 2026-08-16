//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 432/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk432<F: Float>(t2231: F, t27: F, t592: F, t596: F, t21: F, t25: F, t89: F, t90: F, t29: F) -> (F, F, F, F, F, F, F) {
    let t2233 = F::cast_from(30.0_f64) * t2231 * t27;
    let t2235 = F::cast_from(72.0_f64) * t592 * t596;
    let t2236 = t21 * t21;
    let t2237 = F::cast_from(1.0_f64) / t2236;
    let t2239 = F::cast_from(42.0_f64) * t25 * t2237;
    let t2246 = F::cast_from(1.0_f64) / t90 / t89;
    let t2247 = t29 * t2246;
    (t2233, t2235, t2236, t2237, t2239, t2246, t2247)
}
