//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta657 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2386;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta657(t11003: f64, t9303: f64, t10981: f64, t22: f64, t868: f64, t886: f64, t2445: f64, t9292: f64, t588: f64, t780: f64, t39497: f64, t787: f64, t788: f64, t2448: f64, t10994: f64, t2453: f64, t11043: f64, t11029: f64, t39501: f64, t781: f64, t252: f64, t257: f64, t268: f64, t39644: f64, t8779: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40970, t40978, t40988, t40998, t41003) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2386(t11003, t9303, t10981, t22, t868, t886, t2445, t9292, t588, t780, t39497, t787, t788);
        let (t41004, t41011, t41020, t41034, t41037, t41049) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2387(t2448, t9292, t10994, t2453, t11043, t11029, t9303, t39501, t781, t252, t257, t268, t39644, t8779);
    (t40970, t40978, t40988, t40998, t41003, t41004, t41011, t41020, t41034, t41037, t41049)
}
