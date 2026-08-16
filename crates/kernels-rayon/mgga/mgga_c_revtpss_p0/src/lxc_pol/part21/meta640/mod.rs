//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta640 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2415;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2416;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2417;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta640(t11331: f64, t698: f64, t2439: f64, t2912: f64, t11328: f64, t2915: f64, t2909: f64, t11345: f64, t11342: f64, t11821: f64, t240: f64, t2851: f64, t25273: f64, t268: f64, t271: f64, t11161: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41275, t41281, t41283, t41285, t41287, t41289, t41292, t41294, t41295) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2415(t11331, t698, t2439, t2912, t11328, t2915, t2909, t11345, t11342, t11821, t240, t2851);
        let (t41296, t41306) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2416(t41295, t25273, t268, t271);
        let (t41307, t41308) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2417(t41306, t11161, t689);
    (t41275, t41281, t41283, t41285, t41287, t41289, t41292, t41294, t41296, t41306, t41307, t41308)
}
