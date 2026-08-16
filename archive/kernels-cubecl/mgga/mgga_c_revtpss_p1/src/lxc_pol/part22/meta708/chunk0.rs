//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2731/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2731<F: Float>(t1014: F, t65: F, t3252: F, t1513: F, t665: F, t1224: F, t3698: F, t10208: F, t69: F, t1504: F, t658: F, t1509: F, t661: F) -> (F, F, F, F, F, F, F, F) {
    let t27527 = t65 * t1014;
    let t27531 = t65 * t3252;
    let t28036 = t1513 * t665;
    let t29048 = t65 * t1224;
    let t29054 = t65 * t3698;
    let t31035 = t69 * t10208;
    let t31283 = t1504 * t658;
    let t31443 = t1509 * t661;
    (t27527, t27531, t28036, t29048, t29054, t31035, t31283, t31443)
}
