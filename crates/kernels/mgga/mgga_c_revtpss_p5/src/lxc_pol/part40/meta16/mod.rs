//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta16 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk109;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk110;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk111;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk112;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk113;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta16<F: Float>(t273: F, t124: F, t138: F, t139: F, t240: F, t271: F, t276: F, t275: F, t153: F, t159: F, t162: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t279, t281) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk109::<F>(t273, t124, t138);
        let (t282, t283) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk110::<F>(t139, t240, t271);
        let (t285, t287, t290, t291) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk111::<F>(t281, t282, t283, t273, t276, t279);
        let (t293, t300) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk112::<F>(t275, t291, t153, t159, t162, zeta_threshold);
        let t302 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk113::<F>(t273);
    (t279, t281, t282, t283, t285, t287, t290, t291, t293, t300, t302)
}
