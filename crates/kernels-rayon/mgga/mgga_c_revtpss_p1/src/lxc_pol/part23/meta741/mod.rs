//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta741 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2520;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2521;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta741(t14986: f64, t2453: f64, t10506: f64, t2458: f64, t4470: f64, t10069: f64, t14482: f64, t15003: f64, t41020: f64, t786: f64, t867: f64, t14567: f64, t10538: f64, t213: f64, t225: f64, t40321: f64, t14574: f64, t2439: f64, t2777: f64, t14504: f64, t14557: f64, t9303: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51258, t51260, t51263, t51264, t51269, t51276, t51297) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2520(t14986, t2453, t10506, t2458, t4470, t10069, t14482, t15003, t41020, t786, t867, t14567);
        let (t51299, t51320, t51355, t51374, t51390) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2521(t10538, t51297, t213, t225, t40321, t14574, t2439, t2777, t10069, t14504, t14557, t9303);
    (t51258, t51260, t51263, t51264, t51269, t51276, t51297, t51299, t51320, t51355, t51374, t51390)
}
