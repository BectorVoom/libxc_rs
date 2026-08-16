//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta594 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2011;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta594(t94522: f64, t2018: f64, t9646: f64, t9723: f64, t26014: f64, t2689: f64, t3994: f64, t7028: f64, t9845: f64, t25240: f64, t3951: f64, t3964: f64, t2681: f64, t7269: f64, t820: f64, t1416: f64, t240: f64, t25981: f64, t25987: f64, t9775: f64, t2453: f64, t4086: f64, t64: f64, t9795: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94523, t94526, t94527, t94537, t94540) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2011(t94522, t2018, t9646, t9723, t26014, t2689, t3994, t7028, t9845, t25240, t3951, t3964);
        let (t94545, t94546, t94550, t94554, t94564, t94565) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2012(t2681, t7269, t820, t1416, t240, t25981, t25987, t9775, t2453, t4086, t64, t9795);
    (t94523, t94526, t94527, t94537, t94540, t94545, t94546, t94550, t94554, t94564, t94565)
}
