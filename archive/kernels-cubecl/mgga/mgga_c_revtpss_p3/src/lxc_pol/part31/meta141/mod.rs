//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta141 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk751;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk752;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta141<F: Float>(t371: F, t482: F, t676: F, t481: F, t1231: F, t1256: F, t225: F, t3555: F, t480: F, t3566: F, t1236: F, t127: F, t1235: F, t221: F, t462: F, t696: F, t461: F, t1226: F, t140: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3655, t3657, t3658, t3666) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk751::<F>(t371, t482, t676, t481, t1231, t1256, t225, t3555);
        let (t3667, t3670) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk752::<F>(t3666, t480, t225, t3566);
        let (t3671, t3678, t3679, t3682, t3684, t3685) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk753::<F>(t3670, t480, t1236, t127, t371, t1235, t221, t462, t696, t461, t1226, t140);
    (t3655, t3657, t3658, t3666, t3667, t3670, t3671, t3678, t3679, t3682, t3684, t3685)
}
