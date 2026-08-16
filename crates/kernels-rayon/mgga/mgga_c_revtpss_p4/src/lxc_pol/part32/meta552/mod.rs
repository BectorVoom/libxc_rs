//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1869;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta552(t94701: f64, t96204: f64, t26359: f64, t9303: f64, t13790: f64, t4102: f64, t685: f64, t72: f64, t1444: f64, t5740: f64, t675: f64, t14109: f64, t25900: f64, t1892: f64, t786: f64, t25877: f64, t14224: f64, t689: f64, t25304: f64, t27883: f64, t25898: f64, t2453: f64, t1955: f64, t27836: f64, t4075: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96584, t96591, t97680, t97685, t97688) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1869(t94701, t96204, t26359, t9303, t13790, t4102, t685, t72, t1444, t5740, t675, t14109, t25900);
        let (t97700, t97705, t97799, t97802, t97916, t97933) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1870(t1892, t786, t25877, t14224, t689, t25304, t27883, t25898, t2453, t1955, t27836, t4075);
    (t96584, t96591, t97680, t97685, t97688, t97700, t97705, t97799, t97802, t97916, t97933)
}
