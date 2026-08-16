//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2095;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta545(t1320: f64, t6801: f64, t189: f64, t21931: f64, t512: f64, t6800: f64, t749: f64, t13611: f64, t13621: f64, t9398: f64, t9406: f64, t13630: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22191, t22192, t22193, t22194, t22195, t22196, t22197, t22198, t22199, t22200, t22201) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2095(t1320, t6801, t189, t21931, t512, t6800, t749, t13611, t13621, t9398, t9406, t13630);
    (t22191, t22192, t22193, t22194, t22195, t22196, t22197, t22198, t22199, t22200, t22201)
}
