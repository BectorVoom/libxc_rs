//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2061;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta558(t41362: f64, t831: f64, t2628: f64, t2690: f64, t812: f64, t835: f64, t9972: f64, t2617: f64, t9666: f64, t776: f64, t9975: f64, t6589: f64, t67: f64, t246: f64, t22715: f64, t268: f64, t271: f64, t10969: f64, t154: f64, t2769: f64, t885: f64, t9698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41363, t41385, t41414, t41424, t41453, t41466) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2061(t41362, t831, t2628, t2690, t812, t835, t9972, t2617, t9666, t776, t9975, t6589, t67);
        let (t41467, t41654, t41655, t41664, t41666, t41684) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2062(t246, t41466, t22715, t268, t271, t10969, t154, t2769, t885, t9698);
    (t41363, t41385, t41414, t41424, t41453, t41466, t41467, t41654, t41655, t41664, t41666, t41684)
}
