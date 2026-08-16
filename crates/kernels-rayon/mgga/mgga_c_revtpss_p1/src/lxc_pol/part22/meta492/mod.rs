//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta492 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2219;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta492(t13392: f64, t4801: f64, t1042: f64, t11150: f64, t3181: f64, t15936: f64, t4806: f64, t11144: f64, t11852: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16195, t16196, t16199, t16200, t16201, t16204, t16205, t16208) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2219(t13392, t4801, t1042, t11150, t3181, t15936, t4806, t11144, t11852);
    (t16195, t16196, t16199, t16200, t16201, t16204, t16205, t16208)
}
