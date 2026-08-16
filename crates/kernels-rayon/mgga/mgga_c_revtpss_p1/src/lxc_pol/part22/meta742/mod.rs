//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta742 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2808;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2809;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta742(t10988: f64, t2435: f64, t2445: f64, t9292: f64, t11025: f64, t10981: f64, t588: f64, t780: f64, t10991: f64, t39497: f64, t787: f64, t788: f64, t2448: f64, t11036: f64, t10994: f64, t2453: f64, t138: f64, t2438: f64, t2771: f64, t2761: f64, t786: f64, t867: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40986, t40988, t40994, t40998, t40999, t41003) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2808(t10988, t2435, t2445, t9292, t11025, t10981, t588, t780, t10991, t39497, t787, t788);
        let (t41004, t41006, t41011, t41014, t41017) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2809(t2448, t9292, t11036, t2435, t10994, t2453, t138, t2438, t2771, t2761, t786, t867);
    (t40986, t40988, t40994, t40998, t40999, t41003, t41004, t41006, t41011, t41014, t41017)
}
