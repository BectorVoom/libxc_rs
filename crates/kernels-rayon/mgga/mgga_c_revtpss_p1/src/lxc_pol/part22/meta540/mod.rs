//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2350;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2351;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta540(t15936: f64, t17550: f64, t1042: f64, t3708: f64, t5265: f64, t13392: f64, t5302: f64, t1252: f64, t1261: f64, t12956: f64, t17525: f64, t17529: f64, t17536: f64, t17541: f64, t17546: f64, t17547: f64, t3591: f64, t3606: f64, t3613: f64, t3711: f64, t5293: f64, t5299: f64, t1260: f64, t5326: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t17551, t17552, t17556, t17557, t17558, t17561) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2350(t15936, t17550, t1042, t3708, t5265, t13392, t5302, t1252, t1261, t12956, t17525, t17529, t17536, t17541, t17546, t17547, t3591, t3606, t3613, t3711, t5293, t5299);
        let t17569 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2351(t1260, t5326);
    (t17551, t17552, t17556, t17557, t17558, t17561, t17569)
}
