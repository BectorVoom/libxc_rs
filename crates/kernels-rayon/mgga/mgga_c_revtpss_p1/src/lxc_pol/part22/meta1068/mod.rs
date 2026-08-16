//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1068 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3819;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3820;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1068(t47009: f64, t47011: f64, t22461: f64, t4147: f64, t48267: f64, t48269: f64, t47016: f64, t1448: f64, t22287: f64, t39786: f64, t39791: f64, t39795: f64, t4139: f64, t4140: f64, t5536: f64, t5541: f64, t6816: f64, t9547: f64, t30: f64, t47060: f64, t2: f64, t580: f64, t605: f64, t13550: f64, t14: f64, t18280: f64, t21906: f64, t21911: f64, t2257: f64, t27: f64, t3833: f64, t3834: f64, t47025: f64, t48185: f64, t5549: f64, t5824: f64, t6785: f64, t9335: f64, t9342: f64, zeta_threshold: f64, t33: f64, t1113: f64, t13565: f64, t20256: f64, t21918: f64, t21923: f64, t3351: f64, t3841: f64, t3842: f64, t47040: f64, t48212: f64, t5557: f64, t6416: f64, t6792: f64, t9350: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73402, t73403, t73411, t73412, t73416, t73417) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3819(t47009, t47011, t22461, t4147, t48267, t48269, t47016, t1448, t22287, t39786, t39791, t39795, t4139, t4140, t5536, t5541, t6816, t9547);
        let (t73418, t73423, t73444) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3820(t30, t47060, t2, t580, t605, t13550, t14, t18280, t21906, t21911, t2257, t27, t3833, t3834, t47025, t48185, t5549, t5824, t6785, t9335, t9342, zeta_threshold);
        let (t73449, t73470) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3821(t33, t1113, t2, t580, t13565, t14, t20256, t21918, t21923, t27, t3351, t3841, t3842, t47040, t48212, t5557, t6416, t6792, t9342, t9350, zeta_threshold);
    (t73402, t73403, t73411, t73412, t73416, t73417, t73418, t73423, t73444, t73449, t73470)
}
