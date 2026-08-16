//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta387 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1417;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta387(t141: f64, t2908: f64, t41263: f64, t11321: f64, t698: f64, t2297: f64, t2851: f64, t39443: f64, t11341: f64, t11331: f64, t11144: f64, t2439: f64, t2912: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41265, t41267, t41270, t41271, t41273, t41275, t41277, t41279, t41281) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1417(t141, t2908, t41263, t11321, t698, t2297, t2851, t39443, t11341, t11331, t11144, t2439, t2912);
    (t41265, t41267, t41270, t41271, t41273, t41275, t41277, t41279, t41281)
}
