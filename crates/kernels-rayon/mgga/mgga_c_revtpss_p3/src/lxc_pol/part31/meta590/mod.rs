//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2014;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta590(t25240: f64, t3951: f64, t3964: f64, t2681: f64, t7269: f64, t820: f64, t1416: f64, t240: f64, t25981: f64, t25987: f64, t9775: f64, t2453: f64, t4086: f64, t64: f64, t9795: f64, t2018: f64, t40688: f64, t46808: f64, t7256: f64, t9784: f64, t1445: f64, t2439: f64, t25916: f64, t25877: f64, t94390: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94540, t94545, t94546, t94550, t94554, t94564) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2014(t25240, t3951, t3964, t2681, t7269, t820, t1416, t240, t25981, t25987, t9775, t2453, t4086, t64);
        let (t94565, t94569, t94571, t94580, t94589) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2015(t94564, t9795, t2018, t40688, t46808, t7256, t9784, t1445, t2439, t25916, t25877, t94390);
    (t94540, t94545, t94546, t94550, t94554, t94564, t94565, t94569, t94571, t94580, t94589)
}
