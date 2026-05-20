//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta142 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk725;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk726;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta142<F: Float>(t3781: F, t460: F, t3303: F, t471: F, t498: F, t1330: F, t72: F, t757: F, t530: F, t566: F, t525: F, t527: F, t2608: F, t520: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t3782 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk725::<F>(t3781, t460);
        let (t3783, t3800, t3801, t3825, t3826, t3828, t3833, t3841, t3853) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk726::<F>(t3303, t471, t498, t1330, t72, t757, t530, t566, t525, t527, t2608, t520);
    (t3782, t3783, t3800, t3801, t3825, t3826, t3828, t3833, t3841, t3853)
}
