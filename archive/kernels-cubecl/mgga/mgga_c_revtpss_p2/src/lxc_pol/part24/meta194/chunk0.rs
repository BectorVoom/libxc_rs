//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 923/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk923<F: Float>(t1386: F, t2681: F, t820: F, t4000: F, t843: F, t136: F, t4011: F, t240: F, t532: F, t549: F, t72: F, t595: F, t66: F) -> (F, F, F, F, F, F, F, F) {
    let t9909 = t820 * t1386 * t2681;
    let t9918 = t820 * t4000 * t843;
    let t9921 = t4011 * t136;
    let t9934 = t4000 * t240;
    let t9940 = F::cast_from(1.0_f64) / t549 / t532;
    let t9941 = t240 * t9940;
    let t9942 = t9941 * t72;
    let t9948 = F::cast_from(1.0_f64) / t66 / t595;
    (t9909, t9918, t9921, t9934, t9940, t9941, t9942, t9948)
}
