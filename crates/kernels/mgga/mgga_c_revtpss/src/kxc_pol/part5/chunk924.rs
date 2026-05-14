//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 924/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk924<F: Float>(t136: F, t1412: F, t220: F, t1353: F, t4003: F, t2735: F, t4086: F, t3994: F, t808: F, t521: F, t9342: F, t14: F, t588: F, t2496: F, t4038: F, t123: F, t1330: F) -> (F, F, F, F, F, F, F, F) {
    let t9817 = t1412 * t136;
    let t9818 = t9817 * t220;
    let t9835 = t4003 * t1353;
    let t9845 = t2735 * t4086;
    let t9846 = t808 * t3994;
    let t9847 = t9845 * t9846;
    let t9854 = 24.0 * t9342 * t521;
    let t9855 = t14 * t588;
    let t9856 = t9855 * t521;
    let t9858 = t4038 * t2496;
    let t9860 = t1330 * t123;
    (t9818, t9835, t9845, t9847, t9854, t9856, t9858, t9860)
}
