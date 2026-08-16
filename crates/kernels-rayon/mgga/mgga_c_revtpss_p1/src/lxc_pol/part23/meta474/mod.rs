//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1925;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta474(t127: f64, t371: f64, t6337: f64, t3205: f64, t6276: f64, t1025: f64, t4845: f64, t4858: f64, t3172: f64, t6307: f64, t3150: f64, t4820: f64, t4879: f64, t11947: f64, t15745: f64, t16134: f64, t16160: f64, t16190: f64, t1665: f64, t1671: f64, t3188: f64, t6327: f64, t6339: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20016, t20017, t20020, t20021, t20025, t20029, t20030, t20034) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1925(t127, t371, t6337, t3205, t6276, t1025, t4845, t4858, t3172, t6307, t3150, t4820, t4879);
        let t20036 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1926(t11947, t15745, t16134, t16160, t16190, t1665, t1671, t20017, t20021, t20025, t20030, t20034, t3188, t6327, t6339);
    (t20016, t20017, t20020, t20021, t20025, t20029, t20030, t20034, t20036)
}
