//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1556;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1557;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1558;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta303(t10654: f64, t2782: f64, t2760: f64, t822: f64, t2718: f64, t860: f64, t2722: f64, t836: f64, t231: f64, t243: f64, t816: f64, t9707: f64, t813: f64, t2394: f64, t2476: f64, t236: f64, t807: f64, t2689: f64, t2694: f64, t2430: f64, t854: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10655, t10657, t10661, t10665) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1556(t10654, t2782, t2760, t822, t2718, t860, t2722, t836);
        let t10666 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1557(t10665, t231);
        let (t10673, t10674, t10675, t10676, t10678, t10680) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1558(t243, t816, t9707, t813, t2394, t2476, t236, t807, t2689, t2694, t2430, t854);
    (t10655, t10657, t10661, t10665, t10666, t10673, t10674, t10675, t10676, t10678, t10680)
}
