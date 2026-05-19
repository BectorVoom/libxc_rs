//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 976/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk976<F: Float>(t2516: F, t676: F, t3869: F, t2496: F, t1386: F, t2681: F, t820: F, t1401: F, t4000: F, t843: F, t136: F, t4011: F) -> (F, F, F, F, F, F, F, F) {
    let t9863 = t676 * t2516;
    let t9865 = F::cast_from(0.16265371950452609763e-1_f64) * t3869 * t9863;
    let t9866 = t676 * t2496;
    let t9868 = F::cast_from(0.48159733137676571078e0_f64) * t3869 * t9866;
    let t9909 = t820 * t1386 * t2681;
    let t9910 = t9909 * t1401;
    let t9918 = t820 * t4000 * t843;
    let t9921 = t4011 * t136;
    (t9863, t9865, t9866, t9868, t9909, t9910, t9918, t9921)
}
