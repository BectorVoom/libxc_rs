//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta16 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk119;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk120;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk121;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk122;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk123;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta16(t273: f64, t124: f64, t138: f64, t139: f64, t240: f64, t271: f64, t276: f64, t275: f64, t153: f64, t159: f64, t162: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t279, t281) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk119(t273, t124, t138);
        let (t282, t283) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk120(t139, t240, t271);
        let (t285, t287, t290, t291) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk121(t281, t282, t283, t273, t276, t279);
        let (t293, t300) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk122(t275, t291, t153, t159, t162, zeta_threshold);
        let t302 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk123(t273);
    (t279, t281, t282, t283, t285, t287, t290, t291, t293, t300, t302)
}
