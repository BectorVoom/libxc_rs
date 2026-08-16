//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2146;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2147;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta515(t3022: f64, t4729: f64, t15399: f64, t15418: f64, t15420: f64, t15423: f64, t15425: f64, t15427: f64, t15477: f64, t15515: f64, t15549: f64, t15551: f64, t15553: f64, t15555: f64, t15558: f64, t15561: f64, t15571: f64, t15575: f64, t15577: f64, t16179: f64, t1045: f64, t373: f64, t1042: f64, t1041: f64, t11656: f64, t12021: f64, t16140: f64, t16144: f64, t16149: f64, t16154: f64, t16160: f64, t16165: f64, t16167: f64, t16172: f64, t1671: f64, t3124: f64, t3127: f64, t4837: f64, t4869: f64, t4875: f64) -> (f64, f64, f64, f64, f64) {
        let (t16181, t16182) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2146(t3022, t4729, t15399, t15418, t15420, t15423, t15425, t15427, t15477, t15515, t15549, t15551, t15553, t15555, t15558, t15561, t15571, t15575, t15577);
        let t16183 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2147(t16179, t16182);
        let (t16185, t16186, t16189) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2148(t1045, t16183, t373, t1042, t1041, t11656, t12021, t16140, t16144, t16149, t16154, t16160, t16165, t16167, t16172, t1671, t3124, t3127, t4837, t4869, t4875);
    (t16181, t16183, t16185, t16186, t16189)
}
