//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2539;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2540;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta625(t15935: f64, t19661: f64, t1042: f64, t19666: f64, t4801: f64, t1592: f64, t16138: f64, t19399: f64, t247: f64, t3116: f64, t18942: f64, t4915: f64, t1011: f64, t1063: f64, t11656: f64, t11994: f64, t11999: f64, t16057: f64, t16062: f64, t16064: f64, t3127: f64, t4837: f64, t6263: f64, t6312: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19929, t19930, t19933, t19934, t19939, t19940, t19944, t19947) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2539(t15935, t19661, t1042, t19666, t4801, t1592, t16138, t19399, t247, t3116, t18942, t4915);
        let t19950 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2540(t1011, t1063, t11656, t11994, t11999, t16057, t16062, t16064, t19930, t19934, t19940, t19944, t19947, t3127, t4837, t6263, t6312);
    (t19929, t19930, t19933, t19934, t19939, t19940, t19944, t19947, t19950)
}
