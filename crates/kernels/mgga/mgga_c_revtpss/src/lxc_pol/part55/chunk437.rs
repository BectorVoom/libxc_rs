//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 437/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk437<F: Float>(t2044: F, t2170: F, t573: F, t10: F, t17: F, t15: F, t22: F, t11: F, t14: F, t20: F, t27: F, t12: F, t19: F) -> (F, F, F, F, F, F) {
    let t2172 = t2170 * t573 + t2044;
    let t2219 = F::new(2.0) * t10 * t17;
    let t2223 = F::new(6.0) * t15 * t22;
    let t2224 = t11 * t14;
    let t2226 = F::new(12.0) * t2224 * t22;
    let t2230 = F::new(20.0) * t20 * t27;
    let t2231 = t12 * t19;
    (t2172, t2219, t2223, t2226, t2230, t2231)
}
