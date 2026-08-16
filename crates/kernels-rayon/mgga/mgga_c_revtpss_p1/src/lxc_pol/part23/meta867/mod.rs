//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta867 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2762;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2763;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta867(t3938: f64, t73856: f64, t9816: f64, t9818: f64, t1412: f64, t6843: f64, t2661: f64, t3992: f64, t1399: f64, t22020: f64, t46766: f64, t6864: f64, t5675: f64, t9934: f64, t22267: f64, t9976: f64, t13847: f64, t73731: f64, t22294: f64, t48862: f64, t48999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73859, t73920, t73923, t73927, t73929) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2762(t3938, t73856, t9816, t9818, t1412, t6843, t2661, t3992, t1399, t22020, t46766, t6864);
        let (t73951, t73953, t73963, t73975) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2763(t22020, t2661, t5675, t9934, t22267, t9976, t13847, t1399, t73731, t9816, t22294, t48862, t48999);
    (t73859, t73920, t73923, t73927, t73929, t73951, t73953, t73963, t73975)
}
