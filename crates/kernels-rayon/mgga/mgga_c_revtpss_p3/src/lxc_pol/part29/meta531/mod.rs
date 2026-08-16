//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta531 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1860;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta531(t2247: f64, t2251: f64, t68: f64, t26205: f64, t6963: f64, t45972: f64, t7342: f64, t10309: f64, t26178: f64, t25159: f64, t2047: f64, t92569: f64, t116: f64, t26209: f64, t94973: f64, t26375: f64, t531: f64, t198: f64, t206: f64, t7427: f64, t2411: f64, t26580: f64, t25373: f64, t26550: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95310, t95314, t95316, t95319, t95320, t95340) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1860(t2247, t2251, t68, t26205, t6963, t45972, t7342, t10309, t26178, t25159, t2047, t92569);
        let (t95357, t95397, t95464, t95511, t95527, t95536) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1861(t116, t26209, t94973, t26375, t531, t198, t206, t7427, t2411, t26580, t25373, t26550);
    (t95310, t95314, t95316, t95319, t95320, t95340, t95357, t95397, t95464, t95511, t95527, t95536)
}
