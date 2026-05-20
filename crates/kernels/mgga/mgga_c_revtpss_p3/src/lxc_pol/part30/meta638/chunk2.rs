//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2210/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2210<F: Float>(t2327: F, t8151: F, t10301: F, t29411: F, t2247: F, t29362: F, t38: F, t1923: F, t25102: F, t25110: F, t25114: F, t25117: F, t25150: F, t26782: F, t26789: F, t28089: F, t29372: F, t29375: F, t29412: F, t6954: F, t6960: F, t7575: F, t7709: F, t7719: F, t8144: F, t8147: F) -> (F, F) {
    let t104163 = t8151 * t2327;
    let t104181 = t10301 * t29411;
    let t104185 = t2247 * t38 * t29362;
    let t104194 = -t25150 * t8147 / F::new(6.0) - t6954 * t29372 / F::new(3.0) - t6954 * t29375 / F::new(3.0) - t1923 * t26782 * t7719 / F::new(6.0) - t1923 * t7575 * t28089 / F::new(3.0) + t25117 * t8144 / F::new(3.0) + t7709 * t26789 / F::new(3.0) + F::new(5.0) / F::new(3.0) * t104181 * t6960 + F::new(5.0) / F::new(3.0) * t104185 * t6960 + F::new(5.0) / F::new(3.0) * t29412 * t25110 + F::new(5.0) / F::new(6.0) * t29412 * t25114 + F::new(2.0) / F::new(3.0) * t25102 * t8144;
    (t104163, t104194)
}
