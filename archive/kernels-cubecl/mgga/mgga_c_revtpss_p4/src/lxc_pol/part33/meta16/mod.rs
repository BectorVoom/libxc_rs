//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta16 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk120;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk121;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk122;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk123;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk124;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta16<F: Float>(t273: F, t124: F, t138: F, t139: F, t240: F, t271: F, t276: F, t275: F, t153: F, t159: F, t162: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t279, t281) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk120::<F>(t273, t124, t138);
        let (t282, t283) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk121::<F>(t139, t240, t271);
        let (t285, t287, t290, t291) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk122::<F>(t281, t282, t283, t273, t276, t279);
        let (t293, t300) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk123::<F>(t275, t291, t153, t159, t162, zeta_threshold);
        let t302 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk124::<F>(t273);
    (t279, t281, t282, t283, t285, t287, t290, t291, t293, t300, t302)
}
