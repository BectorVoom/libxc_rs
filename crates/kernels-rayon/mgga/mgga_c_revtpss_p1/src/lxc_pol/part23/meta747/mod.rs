//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta747 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2535;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2536;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta747(t2435: f64, t4575: f64, t51973: f64, t52035: f64, t2852: f64, t373: f64, t2439: f64, t4628: f64, t1606: f64, t9303: f64, t2923: f64, t4587: f64, t11384: f64, t1596: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t52037 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2535(t2435, t4575);
        let (t52082, t52091, t52092, t52110, t52126, t52127, t52128, t52219, t52224) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2536(t51973, t52035, t52037, t2852, t373, t2439, t4628, t1606, t9303, t2923, t4587, t11384, t1596);
    (t52037, t52082, t52091, t52092, t52110, t52126, t52127, t52128, t52219, t52224)
}
