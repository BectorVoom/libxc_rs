//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta242 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta242(t14362: f64, t2630: f64, t1469: f64, t2609: f64, t706: f64, t1568: f64, t785: f64, t780: f64, t2439: f64, t1579: f64, t2769: f64, t2470: f64, t4480: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14363, t14440, t14441, t14472, t14473, t14474, t14480, t14485) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1004(t14362, t2630, t1469, t2609, t706, t1568, t785, t780, t2439, t1579, t2769, t2470, t4480);
    (t14363, t14440, t14441, t14472, t14473, t14474, t14480, t14485)
}
