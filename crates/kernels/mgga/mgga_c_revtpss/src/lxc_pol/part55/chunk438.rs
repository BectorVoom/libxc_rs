//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 438/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk438<F: Float>(t2231: F, t27: F, t21: F, t25: F, t599: F, t602: F, t89: F, t90: F, t29: F) -> (F, F, F, F, F) {
    let t2233 = F::new(30.0) * t2231 * t27;
    let t2236 = t21 * t21;
    let t2237 = F::new(1.0) / t2236;
    let t2239 = F::new(42.0) * t25 * t2237;
    let t2242 = t599 * t602;
    let t2246 = F::new(1.0) / t90 / t89;
    let t2247 = t29 * t2246;
    (t2233, t2239, t2242, t2246, t2247)
}
