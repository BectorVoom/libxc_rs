//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 965/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk965<F: Float>(t72: F, t8094: F, t686: F, t25878: F, t25895: F, t27884: F, t7515: F, t212: F, t8085: F, t1358: F, t689: F, t786: F, t8086: F, t1364: F, t8103: F, t7284: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28779 = t8094 * t72;
    let t28780 = t28779 * t686;
    let t28781 = t25878 * t28780;
    let t28783 = t25895 * t28780;
    let t28796 = t27884 * t7515;
    let t28824 = t212 * t8085;
    let t28825 = t28824 * t1358;
    let t28826 = t689 * t28825;
    let t28837 = t786 * t8086;
    let t28838 = t28837 * t1364;
    let t28844 = t8103 * t72;
    let t28845 = t28844 * t686;
    let t28846 = t7284 * t28845;
    (t28779, t28780, t28781, t28783, t28796, t28824, t28825, t28826, t28837, t28838, t28844, t28845, t28846)
}
