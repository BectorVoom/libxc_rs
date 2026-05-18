//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 342/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk342<F: Float>(t1171: F, t536: F, t1117: F, t1123: F, t1129: F, t1130: F, t1134: F, t1139: F, t1144: F, t1146: F, t1149: F, t1150: F, t1158: F, t1163: F, t1167: F, t1169: F, t510: F, t513: F, t518: F, t521: F) -> (F, F) {
    let t1172 = t536 * t1171;
    let t1175 = F::new(2.0) * t1117 * t513 * t1123 - F::new(2.0) * t510 * t1130 + F::new(6.0) * t1134 * t521 * t1123 - F::new(6.0) * t518 * t1139 * t1129 + F::new(3.0) * t1144 * t1146 - F::new(3.0) * t1149 * t1150 - F::new(4.0) / F::new(9.0) * t1158 * t1163 + t1167 * t1146 - t1169 * t1150 - F::new(4.0) / F::new(9.0) * t1172 * t1163;
    (t1172, t1175)
}
