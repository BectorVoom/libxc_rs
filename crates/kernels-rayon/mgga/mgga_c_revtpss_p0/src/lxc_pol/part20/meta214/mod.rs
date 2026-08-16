//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta214 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk997;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk998;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk999;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta214(t10430: f64, t10432: f64, t10435: f64, t10438: f64, t10442: f64, t10444: f64, t10469: f64, t10489: f64, t198: f64, t765: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64, t2828: f64, t886: f64, t2770: f64, t2435: f64, t2445: f64, t2441: f64, t9303: f64, t10115: f64, t258: f64, t2453: f64, t2464: f64, t2438: f64, t138: f64, t2434: f64, t123: f64, t2465: f64, t213: f64, t2760: f64, t215: f64, t231: f64, t268: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t10493 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk997(t10430, t10432, t10435, t10438, t10442, t10444, t10469, t10489, t198, t765, t9278, t9308, t9316, t9329, t9333);
        let (t10495, t10498, t10501, t10503, t10504, t10505) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk998(t2828, t886, t2770, t2435, t2445, t2441, t9303, t10115, t258, t2453, t2464, t2438);
        let (t10506, t10507, t10510, t10511, t10513, t10518) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk999(t10505, t138, t10504, t2434, t886, t123, t2465, t213, t2760, t215, t231, t268, t836);
    (t10493, t10495, t10498, t10501, t10503, t10504, t10506, t10507, t10510, t10511, t10513, t10518)
}
