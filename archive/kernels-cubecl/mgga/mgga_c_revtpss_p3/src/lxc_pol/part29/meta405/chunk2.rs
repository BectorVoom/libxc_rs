//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1465/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1465<F: Float>(t15191: F, t15197: F, t11134: F, t11136: F, t11138: F, t11140: F, t11334: F, t11338: F, t11339: F, t11366: F, t11368: F, t15221: F, t15230: F) -> (F, F, F) {
    let t15457 = F::cast_from(0.19931111111111111111e0_f64) * t15191;
    let t15459 = F::cast_from(0.10954222222222222222e0_f64) * t15197;
    let t15472 = -t11334 - t11338 + F::cast_from(0.3071625e0_f64) * t15221 + F::cast_from(0.18257037037037037037e-1_f64) * t11339 - F::cast_from(0.19931111111111111111e0_f64) * t11138 - F::cast_from(0.26574814814814814816e0_f64) * t11134 + F::cast_from(0.99655555555555555557e-1_f64) * t11140 + F::cast_from(0.66437037037037037038e-1_f64) * t11136 - F::cast_from(0.18257037037037037037e0_f64) * t11366 + F::cast_from(0.54771111111111111111e-1_f64) * t11368 + F::cast_from(0.1898925e1_f64) * t15230;
    (t15457, t15459, t15472)
}
