//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 590/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk590<F: Float>(t2113: F, t2115: F, t572: F, t573: F, t10: F, t17: F, t576: F, t580: F, t15: F, t22: F, t11: F, t14: F) -> (F, F, F, F, F) {
    let t2118 = t2113 * t573 + F::new(3.0) * t2115 * t572;
    let t2219 = F::new(2.0) * t10 * t17;
    let t2221 = F::new(8.0) * t576 * t580;
    let t2223 = F::new(6.0) * t15 * t22;
    let t2224 = t11 * t14;
    (t2118, t2219, t2221, t2223, t2224)
}
