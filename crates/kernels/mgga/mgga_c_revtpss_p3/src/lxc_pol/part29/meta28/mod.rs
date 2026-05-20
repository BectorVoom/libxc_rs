//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta28 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk190;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk191;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk192;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk193;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk194;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk195;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk196;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta28<F: Float>(t539: F, t541: F, t225: F, t235: F, t213: F, t531: F, t241: F, t247: F, t217: F, t535: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t543 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk190::<F>(t539, t541);
        let (t544, t545) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk191::<F>(t543);
        let t546 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk192::<F>(t225, t545);
        let t547 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk193::<F>(t235, t546);
        let (t548, t549, t550) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk194::<F>(t213, t547, t531);
        let (t552, t555) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk195::<F>(t241, t550, t247, t217, t535, t548);
        let t556 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk196::<F>(t225, t555);
    (t543, t544, t545, t546, t547, t548, t549, t550, t552, t555, t556)
}
