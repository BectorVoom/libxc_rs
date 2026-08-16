//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta549(t1042: f64, t17221: f64, t3172: f64, t5269: f64, t1261: f64, t13396: f64, t5268: f64, t12256: f64, t13099: f64, t15936: f64, t1224: f64, t140: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17222, t17225, t17227, t17231, t17232, t17235, t17236, t17237, t17240) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2223(t1042, t17221, t3172, t5269, t1261, t13396, t5268, t12256, t13099, t15936, t1224, t140);
    (t17222, t17225, t17227, t17231, t17232, t17235, t17236, t17237, t17240)
}
