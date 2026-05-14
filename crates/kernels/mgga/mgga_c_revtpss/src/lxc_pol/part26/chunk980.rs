//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 980/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk980<F: Float>(t26379: F, t26702: F, t3: F, t2055: F, t2327: F, t116: F, t7373: F, t670: F, t2371: F, t7553: F, t117: F, t26153: F, t1459: F, t1461: F, t2113: F, t2115: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, t7547: F, t7554: F, t7557: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26703 = t26379 + t26702;
    let t26704 = t3 * t26703;
    let t26716 = param_d * t26703;
    let t26730 = t2327 * t2055;
    let t26733 = t116 * t7373;
    let t26734 = t26733 * t670;
    let t26737 = t7553 * t2371;
    let t26740 = t117 * t26153;
    let t26743 = 12.0 * t1459 * t7554 + 6.0 * t1459 * t7557 + 6.0 * t1461 * t7547 + 6.0 * t2113 * t4162 + 3.0 * t2113 * t4165 + 3.0 * t2115 * t4158 + t26716 * t573 + 6.0 * t26730 * t572 + 12.0 * t26734 * t572 + 6.0 * t26737 * t572 + 3.0 * t26740 * t572;
    (t26703, t26704, t26716, t26730, t26733, t26734, t26737, t26740, t26743)
}
