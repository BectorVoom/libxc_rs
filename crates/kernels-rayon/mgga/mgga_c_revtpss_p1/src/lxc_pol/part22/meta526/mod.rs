//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2308;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta526(t15936: f64, t17202: f64, t1042: f64, t3172: f64, t5298: f64, t3711: f64, t1469: f64, t3568: f64, t5296: f64, t5278: f64, t1250: f64, t17170: f64, t482: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17203, t17204, t17209, t17211, t17212, t17213, t17214, t17217, t17219, t17221) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2308(t15936, t17202, t1042, t3172, t5298, t3711, t1469, t3568, t5296, t5278, t1250, t17170, t482);
    (t17203, t17204, t17209, t17211, t17212, t17213, t17214, t17217, t17219, t17221)
}
