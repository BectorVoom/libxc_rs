//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 934/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk934<F: Float>(t10288: F, t2237: F, t592: F, t2236: F, t3: F, t25: F, t88: F, t89: F, t90: F, t29: F, t46: F, t47: F) -> (F, F, F, F, F, F, F, F) {
    let t10289 = F::new(540.0) * t10288;
    let t10290 = t592 * t2237;
    let t10291 = F::new(756.0) * t10290;
    let t10292 = t2236 * t3;
    let t10293 = F::new(1.0) / t10292;
    let t10295 = F::new(336.0) * t25 * t10293;
    let t10308 = F::new(1.0) / t90 / t89 / t88;
    let t10309 = t29 * t10308;
    let t10355 = F::new(1.0) / t47 / t46;
    (t10289, t10291, t10292, t10293, t10295, t10308, t10309, t10355)
}
