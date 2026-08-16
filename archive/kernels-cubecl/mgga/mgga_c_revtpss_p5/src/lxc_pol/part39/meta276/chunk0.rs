//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1014/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1014<F: Float>(t2735: F, t4086: F, t3994: F, t808: F, t521: F, t9342: F, t14: F, t588: F, t2496: F, t4038: F, t123: F, t1330: F) -> (F, F, F, F, F, F) {
    let t9845 = t2735 * t4086;
    let t9846 = t808 * t3994;
    let t9847 = t9845 * t9846;
    let t9854 = F::cast_from(24.0_f64) * t9342 * t521;
    let t9855 = t14 * t588;
    let t9856 = t9855 * t521;
    let t9858 = t4038 * t2496;
    let t9860 = t1330 * t123;
    (t9845, t9847, t9854, t9856, t9858, t9860)
}
