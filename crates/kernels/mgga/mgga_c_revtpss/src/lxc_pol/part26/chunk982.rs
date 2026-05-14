//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 982/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk982<F: Float>(t2013: F, t28166: F, t8995: F, t2106: F, t9593: F, t198: F, t205: F, t2070: F, t2061: F, t2718: F, t2075: F, t2051: F, t670: F, t2097: F, t3999: F, t2055: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28167 = t2013 * t28166;
    let t28196 = t2013 * t8995;
    let t28286 = t2106 * t9593;
    let t28291 = t198 * t205 * t2070;
    let t28425 = t2718 * t2061;
    let t28472 = t198 * t2075;
    let t28658 = t2051 * t670;
    let t28911 = t3999 * t2097;
    let t28974 = t670 * t2055;
    (t28167, t28196, t28286, t28291, t28425, t28472, t28658, t28911, t28974)
}
