//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta139 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk742;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk743;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta139<F: Float>(t3566: F, t487: F, t1209: F, t1269: F, t3356: F, t3140: F, t460: F, t1242: F, t472: F, t474: F, t3147: F, t479: F, t471: F, t3153: F, t1244: F, t1121: F, t414: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3567, t3572, t3579, t3594, t3596, t3597, t3598) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk742::<F>(t3566, t487, t1209, t1269, t3356, t3140, t460, t1242, t472, t474, t3147, t479);
        let (t3599, t3600, t3603) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk743::<F>(t3597, t3598, t3594, t471);
        let (t3604, t3609, t3610, t3611, t3617) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk744::<F>(t3153, t3603, t1244, t3598, t3594, t471, t1121, t414);
    (t3567, t3572, t3579, t3596, t3599, t3600, t3603, t3604, t3609, t3610, t3611, t3617)
}
