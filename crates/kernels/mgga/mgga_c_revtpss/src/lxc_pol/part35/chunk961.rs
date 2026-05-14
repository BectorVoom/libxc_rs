//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 961/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk961<F: Float>(t197: F, t530: F, t2013: F, t8995: F, t2106: F, t9593: F, t198: F, t205: F, t2070: F, t72: F, t8006: F, t686: F, t25375: F, t25387: F, t27216: F, t7407: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28166 = t197 * t530;
    let t28167 = t2013 * t28166;
    let t28196 = t2013 * t8995;
    let t28286 = t2106 * t9593;
    let t28291 = t198 * t205 * t2070;
    let t28313 = t8006 * t72;
    let t28314 = t28313 * t686;
    let t28315 = t25375 * t28314;
    let t28317 = t25387 * t28314;
    let t28352 = t27216 * t7407;
    (t28167, t28196, t28286, t28291, t28313, t28314, t28315, t28317, t28352)
}
