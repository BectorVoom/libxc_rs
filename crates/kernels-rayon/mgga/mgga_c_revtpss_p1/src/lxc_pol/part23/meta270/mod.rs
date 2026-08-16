//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1480;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1481;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta270(t10199: f64, t2851: f64, t78: f64, t3361: f64, t81: f64, t157: f64, t36: f64, t200: f64, t45: f64, t202: f64, t57: f64, t2435: f64, t2445: f64, t2441: f64, t9303: f64, t10115: f64, t258: f64, t2453: f64, t2464: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10379, t10389, t10398, t10439, t10446, t10457, t10498) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1480(t10199, t2851, t78, t3361, t81, t157, t36, t200, t45, t202, t57, t2435, t2445);
        let (t10501, t10503, t10504) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1481(t2441, t9303, t10115, t258, t2453, t2464);
    (t10379, t10389, t10398, t10439, t10446, t10457, t10498, t10501, t10503, t10504)
}
