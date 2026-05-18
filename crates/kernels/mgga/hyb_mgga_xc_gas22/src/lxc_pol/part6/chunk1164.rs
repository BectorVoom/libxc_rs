//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1164/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1164<F: Float>(t136: F, t215: F, t8184: F, t2004: F, t2011: F, t19746: F, t222: F, t226: F, t12: F, t5: F, t231: F, t243: F) -> (F, F, F, F, F, F) {
    let t20578 = F::new(5.0) / F::new(108.0) * t136 * t8184 * t215;
    let t20579 = t2004 * t2011;
    let t20624 = t222 * t19746 * t226;
    let t20625 = F::new(0.31310740740740740741e1) * t20624;
    let t20626 = t12 * t5;
    let t20631 = F::new(1.0) / t231 / t20626 / t243 / t226 / F::new(96.0);
    (t20578, t20579, t20624, t20625, t20626, t20631)
}
