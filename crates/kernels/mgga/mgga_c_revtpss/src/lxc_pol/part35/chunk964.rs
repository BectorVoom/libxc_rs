//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 964/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk964<F: Float>(t198: F, t2075: F, t26179: F, t7706: F, t7349: F, t7709: F, t13272: F, t7342: F, t2047: F, t28150: F, t7702: F, t7348: F, t7719: F, t1923: F, t116: F, t7968: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28472 = t198 * t2075;
    let t28598 = t26179 * t7706;
    let t28600 = t7709 * t7349;
    let t28602 = t13272 * t7342;
    let t28628 = t2047 * t28150;
    let t28638 = t7702 * t7349;
    let t28640 = t7348 * t7719;
    let t28641 = t1923 * t28640;
    let t28653 = t7968 * t116;
    (t28472, t28598, t28600, t28602, t28628, t28638, t28640, t28641, t28653)
}
