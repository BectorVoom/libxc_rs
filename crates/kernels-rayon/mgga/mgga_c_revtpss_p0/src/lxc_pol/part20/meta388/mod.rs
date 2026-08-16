//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1418;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta388(t11328: f64, t698: f64, t2439: f64, t2915: f64, t2909: f64, t11345: f64, t41246: f64, t41250: f64, t41255: f64, t41260: f64, t41265: f64, t41267: f64, t41273: f64, t41275: f64, t41279: f64, t41281: f64, t11342: f64, t11821: f64, t240: f64, t2851: f64, t39443: f64, t141: f64, t39457: f64, t905: f64, t930: f64, t25273: f64, t268: f64, t271: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41283, t41285, t41287, t41289, t41291) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1418(t11328, t698, t2439, t2915, t2909, t11345, t41246, t41250, t41255, t41260, t41265, t41267, t41273, t41275, t41279, t41281);
        let (t41292, t41296, t41297, t41299, t41301, t41303, t41306) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1419(t11342, t698, t11821, t240, t2851, t39443, t141, t39457, t905, t930, t25273, t268, t271);
    (t41283, t41285, t41287, t41289, t41291, t41292, t41296, t41297, t41299, t41301, t41303, t41306)
}
