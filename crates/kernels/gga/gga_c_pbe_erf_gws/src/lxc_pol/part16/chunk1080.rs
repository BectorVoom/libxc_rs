//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1080/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1080<F: Float>(t14767: F, t2373: F, t1113: F, t13781: F, t2352: F, t3972: F, t824: F, t14733: F, t4484: F, t1112: F, t361: F, t51543: F, t13917: F, t9388: F, t1178: F, t13783: F, t8787: F) -> (F, F, F, F, F) {
    let t53126 = t14767 * t2373;
    let t53131 = t3972 * t13781 * t1113 * t824 * t2352;
    let t53134 = t14733 * t4484;
    let t53138 = t361 * t51543 * t1112;
    let t53140 = t13917 * t53138 * t9388;
    let t53152 = t13917 * t1178 * t8787 * t13783;
    (t53126, t53131, t53134, t53140, t53152)
}
