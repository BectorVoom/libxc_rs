//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1189/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1189<F: Float>(t22633: F, t93: F, t22589: F, t94982: F, t25826: F, t75833: F, t22628: F, t6998: F, t1907: F, t6836: F, t1955: F, t22964: F) -> (F, F, F, F, F, F) {
    let t114385 = t93 * t22633;
    let t114394 = t94982 * t22589;
    let t114396 = t25826 * t75833;
    let t114398 = t6998 * t22628;
    let t114452 = t6836 * t1907;
    let t114485 = t1955 * t22964;
    (t114385, t114394, t114396, t114398, t114452, t114485)
}
