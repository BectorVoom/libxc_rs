//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta740 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2518;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2519;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta740(t2475: f64, t808: f64, t11028: f64, t1580: f64, t2439: f64, t10504: f64, t15002: f64, t9285: f64, t10505: f64, t137: f64, t41011: f64, t11015: f64, t4325: f64, t4477: f64, t9292: f64, t14472: f64, t887: f64, t11044: f64, t14485: f64, t15014: f64, t9303: f64, t10510: f64, t14987: f64, t10982: f64, t1568: f64, t9646: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51176, t51199, t51203, t51208, t51211) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2518(t2475, t808, t11028, t1580, t2439, t10504, t15002, t9285, t10505, t137, t41011, t11015, t4325);
        let (t51213, t51217, t51234, t51237, t51240, t51246) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2519(t4477, t9292, t14472, t2439, t887, t11044, t14485, t15014, t9303, t10510, t14987, t10982, t1568, t9646);
    (t51176, t51199, t51203, t51208, t51211, t51213, t51217, t51234, t51237, t51240, t51246)
}
