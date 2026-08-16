//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta669 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2195;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2196;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta669(t4173: f64, t4187: f64, t21698: f64, t603: f64, t5816: f64, t640: f64, t77: f64, t29561: f64, t644: f64, t4241: f64, t7705: f64, t1927: f64, t21804: f64, t76: f64, t2242: f64, t5819: f64, t38: f64, t60670: f64, t13272: f64, t1470: f64, t29543: f64, t1497: f64, t7719: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t108813, t108816, t108864, t108872, t108876, t108879) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2195(t4173, t4187, t21698, t603, t5816, t640, t77, t29561, t644, t4241, t7705, t1927);
        let (t108941, t108945, t108952, t108966, t108975, t108978) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2196(t21804, t76, t2242, t5819, t38, t60670, t13272, t1470, t29543, t644, t77, t1497, t7719);
    (t108813, t108816, t108864, t108872, t108876, t108879, t108941, t108945, t108952, t108966, t108975, t108978)
}
