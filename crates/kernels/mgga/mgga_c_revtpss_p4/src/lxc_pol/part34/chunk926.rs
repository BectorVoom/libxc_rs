//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 926/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk926<F: Float>(t22633: F, t508: F, t1501: F, t5883: F, t10271: F, t10273: F, t10275: F, t10278: F, t10280: F, t10282: F, t10284: F, t10287: F, t10289: F, t10291: F, t10295: F) -> (F, F, F) {
    let t22634 = t508 * t22633;
    let t22639 = t1501 * t5883;
    let t22648 = -t10271 - t10273 - t10275 - t10278 - t10280 - t10282 - t10284 - t10287 - t10289 - t10291 - t10295;
    (t22634, t22639, t22648)
}
