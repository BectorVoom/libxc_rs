//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta566 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2264;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta566(t1121: f64, t1248: f64, t606: f64, t3604: f64, t17353: f64, t372: f64, t5277: f64, t3630: f64, t12784: f64, t12866: f64, t12910: f64, t17619: f64, t17622: f64, t17625: f64, t17629: f64, t17635: f64, t17640: f64, t17646: f64, t17651: f64, t17654: f64, t3625: f64, t5402: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t17655, t17657, t17658, t17661, t17662, t17665) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2264(t1121, t1248, t606, t3604, t17353, t372, t5277, t3630, t12784, t12866, t12910, t17619, t17622, t17625, t17629, t17635, t17640, t17646, t17651, t17654, t3625, t5402);
    (t17655, t17657, t17658, t17661, t17662, t17665)
}
