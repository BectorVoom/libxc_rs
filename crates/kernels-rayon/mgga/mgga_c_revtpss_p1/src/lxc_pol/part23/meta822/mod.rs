//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta822 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2673;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta822(t1065: f64, t372: f64, t6305: f64, t19912: f64, t3241: f64, t1011: f64, t6292: f64, t697: f64, t11922: f64, t19717: f64, t4899: f64, t11675: f64, t19785: f64, t15906: f64, t19753: f64, t20090: f64, t3115: f64, t19649: f64, t11774: f64, t20039: f64, t53405: f64, t19837: f64, t19744: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66187, t66215, t66218, t66221, t66261) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2673(t1065, t372, t6305, t19912, t3241, t1011, t6292, t697, t11922, t19717, t4899, t11675, t19785);
        let (t66288, t66304, t66306, t66328, t66332, t66355) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2674(t11922, t15906, t19753, t20090, t3115, t19649, t372, t11774, t20039, t53405, t19837, t19744);
    (t66187, t66215, t66218, t66221, t66261, t66288, t66304, t66306, t66328, t66332, t66355)
}
