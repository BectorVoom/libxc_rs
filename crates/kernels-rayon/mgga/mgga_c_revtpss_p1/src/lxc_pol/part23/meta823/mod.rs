//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta823 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2675;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2676;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta823(t11922: f64, t20104: f64, t3115: f64, t15618: f64, t15984: f64, t19477: f64, t73: f64, t1011: f64, t15993: f64, t18913: f64, t18904: f64, t53972: f64, t15987: f64, t18942: f64, t15905: f64, t55599: f64, t6258: f64, t905: f64, t11710: f64, t16089: f64, t19706: f64, t16095: f64, t20095: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66362, t66376, t66395, t66403, t66406) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2675(t11922, t20104, t3115, t15618, t15984, t19477, t73, t1011, t15993, t18913, t18904, t53972);
        let (t66423, t66431, t66434, t66467, t66470) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2676(t1011, t15987, t18942, t15905, t55599, t6258, t905, t11710, t16089, t19706, t16095, t20095);
    (t66362, t66376, t66395, t66403, t66406, t66423, t66431, t66434, t66467, t66470)
}
