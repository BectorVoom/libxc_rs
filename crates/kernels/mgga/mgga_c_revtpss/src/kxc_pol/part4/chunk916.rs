//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 916/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk916<F: Float>(t1386: F, t2681: F, t820: F, t1401: F, t4000: F, t843: F, t4006: F, t136: F, t4011: F, t221: F, t3829: F, t3978: F, t3970: F, t3989: F, t4056: F, t550: F) -> (F, F, F, F, F, F, F) {
    let t9909 = t820 * t1386 * t2681;
    let t9910 = t9909 * t1401;
    let t9918 = t820 * t4000 * t843;
    let t9919 = t9918 * t4006;
    let t9921 = t4011 * t136;
    let t9923 = t9921 * t221 * t3829;
    let t9924 = t3978 * t9923;
    let t9926 = t3989 * t3970;
    let t9929 = t550 * t4056;
    (t9909, t9910, t9919, t9921, t9924, t9926, t9929)
}
