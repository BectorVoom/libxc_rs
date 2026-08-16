//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1068 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3819;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3820;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1068<F: Float>(t47009: F, t47011: F, t22461: F, t4147: F, t48267: F, t48269: F, t47016: F, t1448: F, t22287: F, t39786: F, t39791: F, t39795: F, t4139: F, t4140: F, t5536: F, t5541: F, t6816: F, t9547: F, t30: F, t47060: F, t2: F, t580: F, t605: F, t13550: F, t14: F, t18280: F, t21906: F, t21911: F, t2257: F, t27: F, t3833: F, t3834: F, t47025: F, t48185: F, t5549: F, t5824: F, t6785: F, t9335: F, t9342: F, zeta_threshold: F, t33: F, t1113: F, t13565: F, t20256: F, t21918: F, t21923: F, t3351: F, t3841: F, t3842: F, t47040: F, t48212: F, t5557: F, t6416: F, t6792: F, t9350: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t73402, t73403, t73411, t73412, t73416, t73417) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3819::<F>(t47009, t47011, t22461, t4147, t48267, t48269, t47016, t1448, t22287, t39786, t39791, t39795, t4139, t4140, t5536, t5541, t6816, t9547);
        let (t73418, t73423, t73444) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3820::<F>(t30, t47060, t2, t580, t605, t13550, t14, t18280, t21906, t21911, t2257, t27, t3833, t3834, t47025, t48185, t5549, t5824, t6785, t9335, t9342, zeta_threshold);
        let (t73449, t73470) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3821::<F>(t33, t1113, t2, t580, t13565, t14, t20256, t21918, t21923, t27, t3351, t3841, t3842, t47040, t48212, t5557, t6416, t6792, t9342, t9350, zeta_threshold);
    (t73402, t73403, t73411, t73412, t73416, t73417, t73418, t73423, t73444, t73449, t73470)
}
