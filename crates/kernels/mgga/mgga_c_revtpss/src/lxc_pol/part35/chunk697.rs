//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 697/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk697<F: Float>(t220: F, t9817: F, t2735: F, t4086: F, t521: F, t9342: F, t14: F, t588: F, t2516: F, t676: F, t3869: F, t2496: F, t4010: F, t73: F, t1386: F, t2681: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9818 = t9817 * t220;
    let t9845 = t2735 * t4086;
    let t9854 = 24.0 * t9342 * t521;
    let t9855 = t14 * t588;
    let t9856 = t9855 * t521;
    let t9857 = 144.0 * t9856;
    let t9863 = t676 * t2516;
    let t9865 = 0.16265371950452609763e-1 * t3869 * t9863;
    let t9866 = t676 * t2496;
    let t9868 = 0.48159733137676571078e0 * t3869 * t9866;
    let t9880 = t73 * t4010;
    let t9909 = t820 * t1386 * t2681;
    (t9818, t9845, t9854, t9857, t9863, t9865, t9866, t9868, t9880, t9909)
}
