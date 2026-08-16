//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1468;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1469;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta410(t125: f64, t5977: f64, t10786: f64, t2747: f64, t221: f64, t2485: f64, t6022: f64, t10850: f64, t5962: f64, t775: f64, t2477: f64, t828: f64, t14718: f64, t6035: f64, t2662: f64, t2661: f64, t6016: f64, t2749: f64, t14866: f64, t14871: f64, t18411: f64, t18416: f64, t18420: f64, t18424: f64, t2745: f64, t4362: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18426, t18428, t18432, t18433, t18435) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1468(t125, t5977, t10786, t2747, t221, t2485, t6022, t10850, t5962, t775);
        let (t18437, t18440, t18442, t18444, t18446, t18451) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1469(t18435, t2477, t828, t14718, t6035, t2662, t2661, t125, t6016, t2747, t2749, t18426);
        let t18454 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1470(t14866, t14871, t18411, t18416, t18420, t18424, t18428, t18433, t18437, t18442, t18446, t18451, t2745, t4362, t851);
    (t18426, t18428, t18432, t18435, t18437, t18440, t18444, t18446, t18451, t18454)
}
