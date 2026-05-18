//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 995/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk995<F: Float>(t17023: F, t17032: F, t17154: F, t24219: F, t24223: F, t24253: F, t24257: F, t24259: F, t24261: F, t24264: F, t24326: F, t24329: F, t24431: F, t24436: F, t24453: F, t24468: F, t3477: F, t3521: F, t435: F, t5120: F, t6487: F, t6503: F, t6506: F, t6519: F) -> F {
    let t24470 = -F::new(6.0) * t17023 * t6487 + F::new(6.0) * t3477 * t24431 - F::new(0.35089341735807877242e1) * t17154 * t6519 + F::new(0.35089341735807877242e1) * t3521 * t24436 + t24219 - t24223 - t24257 - t24259 - t24261 + t24264 - t24326 - t24329 + F::new(3.0) * t5120 * t6503 + F::new(0.96491876992155210402e2) * t17032 * t6506 - F::new(0.310907e-1) * t24453 * t435 + t24468 - F::new(0.19751673498613801407e-1) * t24253;
    t24470
}
