//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1457/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1457<F: Float>(t11326: F, t15108: F, t15111: F, t15114: F, t15116: F, t15119: F, t15121: F, t15123: F, t15125: F, t15132: F, t15178: F, t15181: F, t15184: F, t15187: F, t15189: F, t15195: F, t15200: F, t15301: F, t15315: F, t15322: F, t15324: F, t15337: F) -> F {
    let t15339 = F::cast_from(0.264729375e1_f64) * t15108 - F::cast_from(0.157790625e0_f64) * t15111 - F::new(0.3529725e1) * t15114 - F::new(0.17648625e1) * t15116 + F::new(0.6311625e0) * t15119 + F::new(0.31558125e0) * t15121 - F::cast_from(0.11577222222222222222e0_f64) * t15123 - F::cast_from(0.68863333333333333333e0_f64) * t15125 + t15301 - F::cast_from(0.68863333333333333334e0_f64) * t15132 + t15315 - F::cast_from(0.34731666666666666667e-1_f64) * t15178 - F::cast_from(0.46308888888888888889e-1_f64) * t15181 + F::new(0.41678e0) * t15184 + F::new(0.20839e0) * t15187 - F::cast_from(0.22954444444444444444e0_f64) * t15189 + t15322 - F::new(0.516475e0) * t15195 + t15324 - F::new(0.104195e0) * t15200 - F::cast_from(0.13892666666666666667e0_f64) * t11326 + t15337;
    t15339
}
