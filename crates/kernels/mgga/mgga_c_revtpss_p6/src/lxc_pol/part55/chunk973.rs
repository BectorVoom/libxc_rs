//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 973/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk973<F: Float>(t212: F, t8085: F, t1358: F, t689: F, t2097: F, t543: F, t5658: F, t7301: F, t786: F, t8086: F, t1364: F, t5774: F) -> (F, F, F, F) {
    let t28824 = t212 * t8085;
    let t28825 = t28824 * t1358;
    let t28826 = t689 * t28825;
    let t28829 = t2097 * t5658 * t543;
    let t28830 = t7301 * t28829;
    let t28837 = t786 * t8086;
    let t28838 = t28837 * t1364;
    let t28840 = t2097 * t5774;
    (t28826, t28830, t28838, t28840)
}
