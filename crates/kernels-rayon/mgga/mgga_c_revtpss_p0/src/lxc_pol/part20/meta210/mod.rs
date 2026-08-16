//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta210 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk987;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta210(t10355: f64, t10356: f64, t2275: f64, t606: f64, t2258: f64, t10326: f64, t48: f64, t58: f64, t59: f64, t2282: f64, t60: f64, t10199: f64, t10345: f64, t2270: f64, t2276: f64, t2279: f64, t44: f64, t49: f64, t56: f64, t614: f64, t617: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10357, t10360, t10361, t10364, t10368, t10369, t10372, t10373, t10376, t10379) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk987(t10355, t10356, t2275, t606, t2258, t10326, t48, t58, t59, t2282, t60, t10199);
        let t10380 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk988(t10345, t10357, t10361, t10364, t10369, t10373, t10376, t10379, t2270, t2276, t2279, t44, t49, t56, t614, t617);
    (t10357, t10360, t10361, t10364, t10368, t10372, t10380)
}
