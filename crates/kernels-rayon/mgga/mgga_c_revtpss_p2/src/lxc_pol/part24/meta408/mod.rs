//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1349;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1350;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta408(t123: f64, t212: f64, t9291: f64, t10981: f64, t588: f64, t780: f64, t39497: f64, t787: f64, t788: f64, t10994: f64, t2453: f64, t39501: f64, t781: f64, t252: f64, t257: f64, t268: f64, t39644: f64, t8779: f64, t11007: f64, t786: f64, t11006: f64, t256: f64, t225: f64, t2441: f64, t39515: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40921, t40998, t41003, t41011, t41037) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1349(t123, t212, t9291, t10981, t588, t780, t39497, t787, t788, t10994, t2453, t39501, t781);
        let (t41049, t41070, t41078, t41095) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1350(t252, t257, t268, t39644, t8779, t11007, t786, t11006, t256, t225, t2441, t39515);
    (t40921, t40998, t41003, t41011, t41037, t41049, t41070, t41078, t41095)
}
