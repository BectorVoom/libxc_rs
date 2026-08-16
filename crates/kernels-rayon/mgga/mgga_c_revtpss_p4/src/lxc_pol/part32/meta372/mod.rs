//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1326;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1327;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta372(t15993: f64, t4574: f64, t1011: f64, t1012: f64, t11821: f64, t11922: f64, t4906: f64, t3115: f64, t4895: f64, t4892: f64, t140: f64, t4886: f64, t3241: f64, t4924: f64, t12047: f64, t15905: f64, t12167: f64, t3057: f64, t380: f64, t3088: f64, t370: f64, t994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15996, t16012, t16037, t16057, t16060) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1326(t15993, t4574, t1011, t1012, t11821, t11922, t4906, t3115, t4895, t4892, t140, t4886);
        let (t16062, t16064, t16067, t16081, t16088, t16089, t16094) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1327(t1011, t16060, t3241, t4924, t12047, t15905, t12167, t3057, t380, t3088, t370, t994);
    (t15996, t16012, t16037, t16057, t16062, t16064, t16067, t16081, t16088, t16089, t16094)
}
