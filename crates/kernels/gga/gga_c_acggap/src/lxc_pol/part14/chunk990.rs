//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 990/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk990<F: Float>(t142: F, t5906: F, t7436: F, t30725: F, t30729: F, t34746: F, t34754: F, t37233: F, t39438: F, t39442: F, t39447: F, t39451: F, t39454: F, t39458: F, t39462: F, t39465: F, t39468: F, t39471: F, t39474: F) -> (F,) {
    let t39477 = t7436 * t142 * t5906;
    let t39479 = -t34746 + 0.52413487149340253447e-3 * t39438 + t37233 + 0.31448092289604152068e-3 * t39442 + t34754 + 0.15724046144802076034e-2 * t30725 + t30729 - 0.15724046144802076034e-2 * t39447 + 0.28582678745379824648e-3 * t39451 + 0.42874018118069736972e-3 * t39454 + 0.62896184579208304136e-3 * t39458 + 0.62896184579208304136e-3 * t39462 - t39465 / 16.0 + t39468 / 8.0 + t39471 / 24.0 + t39474 / 16.0 + t39477 / 48.0;
    (t39479,)
}
