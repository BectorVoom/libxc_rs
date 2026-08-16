//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta738 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2514;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2515;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta738(t50977: f64, t40672: f64, t828: f64, t14819: f64, t40517: f64, t14741: f64, t2710: f64, t2713: f64, t10744: f64, t14861: f64, t808: f64, t40791: f64, t4442: f64, t14742: f64, t2689: f64, t243: f64, t9794: f64, t10760: f64, t14495: f64, t14587: f64, t40799: f64, t4372: f64, t9789: f64, t40627: f64, t50451: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50978, t51014, t51042, t51055, t51059, t51060) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2514(t50977, t40672, t828, t14819, t40517, t14741, t2710, t2713, t10744, t14861, t808, t40791, t4442);
        let (t51061, t51074, t51079, t51081, t51083, t51086) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2515(t51060, t14742, t2689, t243, t9794, t10760, t14495, t14587, t40799, t4372, t9789, t40627, t50451);
    (t50978, t51014, t51042, t51055, t51059, t51061, t51074, t51079, t51081, t51083, t51086)
}
