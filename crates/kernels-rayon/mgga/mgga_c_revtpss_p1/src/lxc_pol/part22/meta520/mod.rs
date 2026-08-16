//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2290;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta520(t1150: f64, t16942: f64, t1131: f64, t1168: f64, t5143: f64, t1745: f64, t3471: f64, t12423: f64, t16649: f64, t16651: f64, t16654: f64, t16657: f64, t16660: f64, t16664: f64, t16667: f64, t16671: f64, t16690: f64, t3452: f64, t5147: f64, t3453: f64, t5146: f64, t3479: f64, t5142: f64, t12472: f64, t1744: f64, t1757: f64, t3497: f64, t1187: f64, t5181: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16943, t16945, t16948, t16951, t16954) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2290(t1150, t16942, t1131, t1168, t5143, t1745, t3471, t12423, t16649, t16651, t16654, t16657, t16660, t16664, t16667, t16671, t16690, t3452, t5147);
        let (t16955, t16959, t16962, t16966, t16971, t16974, t16979) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2291(t3453, t5146, t3479, t5142, t1168, t3471, t12472, t1744, t1757, t3497, t1745, t1187, t5181);
    (t16943, t16945, t16948, t16951, t16954, t16955, t16959, t16962, t16966, t16971, t16974, t16979)
}
