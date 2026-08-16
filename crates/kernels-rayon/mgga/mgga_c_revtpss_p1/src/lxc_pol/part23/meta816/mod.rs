//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta816 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2662;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2663;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta816(t11921: f64, t15716: f64, t19456: f64, t247: f64, t19696: f64, t3168: f64, t15830: f64, t4817: f64, t1063: f64, t11986: f64, t6100: f64, t20054: f64, t3106: f64, t19701: f64, t3127: f64, t3172: f64, t19658: f64, t3169: f64, t19894: f64, t15707: f64, t15734: f64, t19882: f64, t3188: f64, t16190: f64, t4820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65298, t65342, t65347, t65357, t65359) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2662(t11921, t15716, t19456, t247, t19696, t3168, t15830, t4817, t1063, t11986, t6100, t20054, t3106);
        let (t65376, t65431, t65444, t65446, t65454, t65456) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2663(t19701, t3127, t3172, t19658, t3169, t19894, t15707, t15734, t19882, t3188, t16190, t4820);
    (t65298, t65342, t65347, t65357, t65359, t65376, t65431, t65444, t65446, t65454, t65456)
}
