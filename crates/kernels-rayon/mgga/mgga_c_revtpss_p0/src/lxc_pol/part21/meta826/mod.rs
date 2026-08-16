//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta826 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3078;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3079;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta826(t12228: f64, t1732: f64, t44091: f64, t44093: f64, t43748: f64, t5068: f64, t45046: f64, t5109: f64, t12361: f64, t16652: f64, t12243: f64, t16662: f64, t1149: f64, t16943: f64, t3384: f64, t16942: f64, t3433: f64, t3435: f64, t56262: f64, t56264: f64, t56268: f64, t56271: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56275, t56277, t56279, t56281, t56283) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3078(t12228, t1732, t44091, t44093, t43748, t5068, t45046, t5109, t12361, t16652, t12243, t16662);
        let (t56286, t56290, t56291) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3079(t1149, t16943, t3384, t16942, t3433, t3435, t56262, t56264, t56268, t56271, t56275, t56277, t56279, t56281, t56283);
    (t56275, t56277, t56279, t56281, t56283, t56286, t56290, t56291)
}
