//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2388;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta626(t10489: f64, t236: f64, t807: f64, t854: f64, t10681: f64, t2689: f64, t16: f64, t2236: f64, t240: f64, t243: f64, t281: f64, t39644: f64, t2645: f64, t775: f64, t10779: f64, t10786: f64, t14931: f64, t40583: f64, t10773: f64, t10811: f64, t10696: f64, t72: f64, t245: f64, t10729: f64, t9775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40643, t40645, t40649, t40650, t40654) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2388(t10489, t236, t807, t854, t10681, t2689, t16, t2236, t240, t243, t281, t39644);
        let (t40655, t40662, t40669, t40672, t40673, t40679) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2389(t2645, t775, t10779, t10786, t14931, t40583, t10773, t10811, t10696, t72, t245, t10729, t9775);
    (t40643, t40645, t40649, t40650, t40654, t40655, t40662, t40669, t40672, t40673, t40679)
}
