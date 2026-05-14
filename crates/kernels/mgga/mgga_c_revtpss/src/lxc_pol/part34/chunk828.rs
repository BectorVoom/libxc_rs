//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 828/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk828<F: Float>(t3153: F, t6299: F, t73: F, t1065: F, t6244: F, t3172: F, t6301: F, t1041: F, t6258: F, t1032: F, t6235: F, t1040: F, t19463: F, t366: F, t11710: F, t6267: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19572 = t6299 * t3153;
    let t19611 = t6299 * t73;
    let t19649 = t1065 * t6244;
    let t19658 = t3172 * t6301;
    let t19659 = t1041 * t19658;
    let t19675 = t1065 * t6258;
    let t19696 = t6235 * t1032;
    let t19697 = t19696 * t1040;
    let t19773 = t19463 * t366;
    let t19785 = t11710 * t6267;
    (t19572, t19611, t19649, t19658, t19659, t19675, t19696, t19697, t19773, t19785)
}
