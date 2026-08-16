//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta790 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2605;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2606;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta790(t10845: f64, t18531: f64, t18618: f64, t2741: f64, t18622: f64, t6016: f64, t853: f64, t2661: f64, t2662: f64, t2749: f64, t14718: f64, t18637: f64, t50583: f64, t6035: f64, t18408: f64, t837: f64, t18432: f64, t40336: f64, t5977: f64, t10726: f64, t10786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61572, t61574, t61576, t61579, t61582, t61612) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2605(t10845, t18531, t18618, t2741, t18622, t6016, t853, t2661, t2662, t2749, t14718, t18637);
        let (t61616, t61620, t61623, t61625, t61628) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2606(t2661, t2662, t50583, t6035, t18408, t837, t18432, t40336, t5977, t853, t10726, t10786);
    (t61572, t61574, t61576, t61579, t61582, t61612, t61616, t61620, t61623, t61625, t61628)
}
