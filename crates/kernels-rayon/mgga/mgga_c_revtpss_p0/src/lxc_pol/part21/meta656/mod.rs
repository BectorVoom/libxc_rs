//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta656 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2445;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta656(t11804: f64, t11921: f64, t247: f64, t4837: f64, t1063: f64, t11169: f64, t3109: f64, t1011: f64, t11758: f64, t140: f64, t11823: f64, t11828: f64, t11144: f64, t3252: f64, t11852: f64, t126: f64, t11145: f64, t11679: f64, t11710: f64, t3091: f64, t11247: f64, t11249: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42487, t42496, t42499, t42506, t42516) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2445(t11804, t11921, t247, t4837, t1063, t11169, t3109, t1011, t11758, t140, t11823, t11828);
        let (t42518, t42537, t42546, t42550) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2446(t11144, t3252, t11852, t126, t1063, t11145, t247, t11679, t11710, t3091, t11247, t11249);
    (t42487, t42496, t42499, t42506, t42516, t42518, t42537, t42546, t42550)
}
