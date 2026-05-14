//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 769/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk769<F: Float>(t1065: F, t6244: F, t3172: F, t6301: F, t1041: F, t6258: F, t1032: F, t6235: F, t1040: F, t19463: F, t366: F, t11710: F, t6267: F, t3091: F, t6311: F, t3161: F) -> (F, F, F, F, F, F, F) {
    let t19649 = t1065 * t6244;
    let t19658 = t3172 * t6301;
    let t19659 = t1041 * t19658;
    let t19675 = t1065 * t6258;
    let t19696 = t6235 * t1032;
    let t19697 = t19696 * t1040;
    let t19773 = t19463 * t366;
    let t19785 = t11710 * t6267;
    let t19786 = t3091 * t19785;
    let t19826 = t3172 * t6311;
    let t19827 = t3161 * t19826;
    (t19649, t19659, t19675, t19697, t19773, t19786, t19827)
}
