//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1203/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1203<F: Float>(t25944: F, t96255: F, t136: F, t2457: F, t7531: F, t26069: F, t7515: F, t94879: F, t26230: F, t9685: F, t25878: F, t25904: F, t96248: F) -> (F, F, F, F, F, F, F) {
    let t96257 = F::cast_from(0.68540937416128198417e-2_f64) * t25944 * t96255;
    let t96259 = t7531 * t136 * t2457;
    let t96260 = t26069 * t96259;
    let t96262 = t94879 * t7515;
    let t96264 = t26230 * t9685;
    let t96265 = t25878 * t96264;
    let t96269 = t25904 * t96248;
    (t96257, t96259, t96260, t96262, t96264, t96265, t96269)
}
