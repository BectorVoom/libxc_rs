//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta384 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1422;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta384(t1011: f64, t16060: f64, t3241: f64, t4924: f64, t12047: f64, t15905: f64, t12167: f64, t3057: f64, t380: f64, t3088: f64, t370: f64, t994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16062, t16064, t16067, t16081, t16087, t16088, t16089, t16094) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1422(t1011, t16060, t3241, t4924, t12047, t15905, t12167, t3057, t380, t3088, t370, t994);
    (t16062, t16064, t16067, t16081, t16087, t16088, t16089, t16094)
}
