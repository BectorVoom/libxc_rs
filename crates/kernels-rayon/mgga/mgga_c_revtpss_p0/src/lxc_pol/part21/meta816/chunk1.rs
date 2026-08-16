//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2996/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2996(t11806: f64, t15689: f64, t15700: f64, t15701: f64, t15745: f64, t16222: f64, t19878: f64, t3220: f64, t42900: f64, t53729: f64, t53735: f64, t54263: f64, t54656: f64, t54658: f64, t54667: f64, t54672: f64, t54678: f64, t54680: f64) -> f64 {
    let t54684 = 0.42874018118069736972e-3_f64 * t42900 + 0.30488190661738479624e-2_f64 * t54656 - 0.42874018118069736973e-2_f64 * t15700 * t54658 * t53729 + 0.85748036236139473944e-3_f64 * t15689 * t15701 * t53735 + 0.95275595817932748827e-3_f64 * t54667 + 0.71456696863449561621e-3_f64 * t15700 * t16222 * t54263 + 0.19055119163586549765e-2_f64 * t15700 * t54672 * t53729 + 0.12862205435420921092e-2_f64 * t19878 * t11806 + 0.45732285992607719436e-2_f64 * t54678 - 0.85748036236139473944e-3_f64 * t54680 + 0.34299214494455789577e-2_f64 * t15745 * t3220;
    t54684
}
