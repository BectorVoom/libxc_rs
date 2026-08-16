//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1582;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1583;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta427(t1042: f64, t17203: f64, t3172: f64, t5298: f64, t3711: f64, t1469: f64, t3568: f64, t5296: f64, t5278: f64, t1250: f64, t17170: f64, t482: f64, t5269: f64, t1261: f64, t13396: f64, t5268: f64, t12256: f64, t13099: f64, t15936: f64, t1224: f64, t140: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17204, t17211, t17214, t17219, t17221) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1582(t1042, t17203, t3172, t5298, t3711, t1469, t3568, t5296, t5278, t1250, t17170, t482);
        let (t17222, t17227, t17232, t17237, t17240) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1583(t1042, t17221, t3172, t5269, t1261, t13396, t5268, t12256, t13099, t15936, t1224, t140);
    (t17204, t17211, t17214, t17219, t17222, t17227, t17232, t17237, t17240)
}
