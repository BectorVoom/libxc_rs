//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1282/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1282<F: Float>(t3994: F, t808: F, t9845: F, t521: F, t9342: F, t14: F, t588: F, t2496: F, t4038: F, t123: F, t1330: F, t2630: F) -> (F, F, F, F, F) {
    let t9846 = t808 * t3994;
    let t9847 = t9845 * t9846;
    let t9854 = F::new(24.0) * t9342 * t521;
    let t9855 = t14 * t588;
    let t9856 = t9855 * t521;
    let t9858 = t4038 * t2496;
    let t9860 = t1330 * t123;
    let t9861 = t9860 * t2630;
    (t9847, t9854, t9856, t9858, t9861)
}
