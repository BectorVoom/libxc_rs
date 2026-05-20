//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2996/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2996<F: Float>(t11806: F, t15689: F, t15700: F, t15701: F, t15745: F, t16222: F, t19878: F, t3220: F, t42900: F, t53729: F, t53735: F, t54263: F, t54656: F, t54658: F, t54667: F, t54672: F, t54678: F, t54680: F) -> F {
    let t54684 = F::cast_from(0.42874018118069736972e-3_f64) * t42900 + F::cast_from(0.30488190661738479624e-2_f64) * t54656 - F::cast_from(0.42874018118069736973e-2_f64) * t15700 * t54658 * t53729 + F::cast_from(0.85748036236139473944e-3_f64) * t15689 * t15701 * t53735 + F::cast_from(0.95275595817932748827e-3_f64) * t54667 + F::cast_from(0.71456696863449561621e-3_f64) * t15700 * t16222 * t54263 + F::cast_from(0.19055119163586549765e-2_f64) * t15700 * t54672 * t53729 + F::cast_from(0.12862205435420921092e-2_f64) * t19878 * t11806 + F::cast_from(0.45732285992607719436e-2_f64) * t54678 - F::cast_from(0.85748036236139473944e-3_f64) * t54680 + F::cast_from(0.34299214494455789577e-2_f64) * t15745 * t3220;
    t54684
}
