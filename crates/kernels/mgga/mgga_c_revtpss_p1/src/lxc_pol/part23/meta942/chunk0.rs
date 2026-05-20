//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3093/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3093<F: Float>(t1196: F, t20891: F, t24375: F, t43752: F, t16840: F, t20574: F, t17092: F, t20577: F, t1149: F, t12248: F, t24221: F, t3433: F, t5104: F, t6439: F) -> (F, F, F, F, F) {
    let t81589 = F::cast_from(0.12304822629859687989e5_f64) * t1196 * t43752 * t24375 * t20891;
    let t81591 = F::new(18.0) * t16840 * t20574;
    let t81593 = F::new(12.0) * t17092 * t20577;
    let t81596 = F::new(24.0) * t12248 * t24221 * t1149;
    let t81599 = F::new(18.0) * t3433 * t6439 * t5104;
    (t81589, t81591, t81593, t81596, t81599)
}
