//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 797/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk797<F: Float>(t548: F, t9951: F, t4010: F, t72: F, t245: F, t1386: F, t820: F, t844: F, t2482: F, t596: F, t1384: F, t235: F) -> (F, F, F, F, F, F) {
    let t9953 = F::cast_from(0.37792653007779990369e-1_f64) * t548 * t9951;
    let t9954 = t4010 * t72;
    let t9955 = t9954 * t245;
    let t9962 = t820 * t1386 * t844;
    let t9976 = t2482 * t1386 * t596;
    let t9989 = t1384 * t1384;
    let t9990 = F::cast_from(1.0_f64) / t9989;
    let t9991 = t9990 * t235;
    (t9953, t9955, t9962, t9976, t9990, t9991)
}
