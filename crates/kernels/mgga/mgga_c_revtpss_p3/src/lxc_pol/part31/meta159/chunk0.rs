//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 800/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk800<F: Float>(t38: F, t4217: F, t1469: F, t2299: F, t4186: F, t633: F, t2306: F, t637: F, t606: F, t77: F, t1471: F, t1487: F, t1494: F, t4182: F, t4188: F, t4191: F, t4196: F, t608: F, t628: F, t641: F, t71: F, t85: F) -> (F, F, F, F, F, F) {
    let t4218 = t38 * t4217;
    let t4227 = t2299 * t1469;
    let t4230 = t633 * t4186;
    let t4232 = t2306 * t1469;
    let t4235 = t637 * t4186;
    let t4237 = F::new(28.0) / F::new(9.0) * t4227 * t606 - F::new(4.0) / F::new(3.0) * t4230 + F::new(28.0) / F::new(9.0) * t4232 * t606 + F::new(4.0) / F::new(3.0) * t4235;
    let t4238 = t77 * t4237;
    let t4241 = -t4182 * t85 / F::new(12.0) - t4188 * t85 / F::new(12.0) - t4191 * t85 / F::new(12.0) - t1471 * t641 / F::new(12.0) - t4196 * t85 / F::new(12.0) + t4218 * t85 / F::new(24.0) + t1487 * t641 / F::new(24.0) - t608 * t1494 / F::new(12.0) + t628 * t1494 / F::new(24.0) + t71 * t4238 / F::new(24.0);
    (t4218, t4227, t4232, t4237, t4238, t4241)
}
