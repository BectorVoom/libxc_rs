//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1608/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1608<F: Float>(t221: F, t6836: F, t9921: F, t3978: F, t125: F, t6816: F, t1399: F, t3936: F, t6843: F, t3938: F, t5673: F, t21990: F, t5674: F) -> (F, F, F, F, F, F, F) {
    let t22068 = t9921 * t221 * t6836;
    let t22069 = t3978 * t22068;
    let t22074 = t125 * t6816;
    let t22076 = t3936 * t22074 * t1399;
    let t22079 = t125 * t6843;
    let t22081 = t3936 * t22079 * t3938;
    let t22085 = t5673 * t22079 * t1399;
    let t22089 = t5673 * t5674 * t21990;
    (t22068, t22069, t22076, t22079, t22081, t22085, t22089)
}
