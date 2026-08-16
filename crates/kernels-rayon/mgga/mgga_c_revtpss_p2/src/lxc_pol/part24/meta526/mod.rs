//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1558;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta526(t12772: f64, t24786: f64, t3625: f64, t17572: f64, t21188: f64, t13052: f64, t24667: f64, t3172: f64, t12916: f64, t24705: f64, t3718: f64, t1222: f64, t17240: f64, t24244: f64, t24648: f64, t3711: f64, t1261: f64, t24228: f64, t247: f64, t44895: f64, t20820: f64, t5265: f64, t20851: f64, t5362: f64, t21101: f64, t5273: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t83435, t83462, t83485, t83490, t83504) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1558(t12772, t24786, t3625, t17572, t21188, t13052, t24667, t3172, t12916, t24705, t3718, t1222, t17240, t24244);
        let (t83539, t83558, t83580, t83584, t83603) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1559(t24648, t3172, t3711, t1261, t24228, t247, t44895, t20820, t5265, t20851, t5362, t21101, t5273);
    (t83435, t83462, t83485, t83490, t83504, t83539, t83558, t83580, t83584, t83603)
}
