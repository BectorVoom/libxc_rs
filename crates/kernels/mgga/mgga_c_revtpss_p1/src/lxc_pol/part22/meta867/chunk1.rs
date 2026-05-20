//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3024/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3024<F: Float>(t14362: F, t9572: F, t37: F, t4391: F, t14767: F, t221: F, t10703: F, t2674: F, t2661: F, t2662: F, t2754: F, t4352: F) -> (F, F, F, F) {
    let t50901 = t14362 * t9572;
    let t50903 = t37 * t4391;
    let t50931 = t221 * t14767;
    let t50933 = t2674 * t10703 * t50931;
    let t50937 = t2661 * t2662 * t4352 * t2754;
    (t50901, t50903, t50933, t50937)
}
