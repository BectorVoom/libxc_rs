//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta495 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2224;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2225;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2226;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta495(t15717: f64, t996: f64, t1678: f64, t3057: f64, t15648: f64, t16152: f64, t15837: f64, t4930: f64, t994: f64, t3046: f64, t1000: f64, t11187: f64, t11201: f64, t11220: f64, t1680: f64, t1696: f64, t3043: f64, t3047: f64, t3058: f64, t3060: f64, t3063: f64, t3264: f64, t3271: f64, t4752: f64, t4758: f64, t4764: f64, t4773: f64, t4941: f64, t4947: f64, t995: f64, t379: f64, t1078: f64, t1651: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16275, t16284, t16287, t16292, t16295, t16302, t16305, t16310) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2224(t15717, t996, t1678, t3057, t15648, t16152, t15837, t4930, t994, t3046, t1000, t11187, t11201, t11220, t1680, t1696, t3043, t3047, t3058, t3060, t3063, t3264, t3271, t4752, t4758, t4764, t4773, t4941, t4947, t995);
        let t16312 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2225(t3057, t379);
        let t16313 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2226(t1078, t1651);
    (t16275, t16284, t16287, t16292, t16295, t16302, t16305, t16310, t16312, t16313)
}
