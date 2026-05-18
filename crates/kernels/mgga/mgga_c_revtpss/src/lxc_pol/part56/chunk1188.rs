//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1188/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1188<F: Float>(t29127: F, t33468: F, t105270: F, t124604: F, t124605: F, t124626: F, t124665: F, t124671: F, t124819: F, t124915: F, t124927: F, t1294: F, t131631: F, t131657: F, t26948: F, t29179: F, t31993: F, t33471: F, t33477: F, t33478: F, t34908: F, t34940: F, t34960: F, t3719: F, t5230: F, t5422: F, t7637: F, t7652: F, t8945: F, t8948: F) -> F {
    let t131826 = t33468 * t29127;
    let t131849 = F::new(0.6854368519812282314e1) * t33477 * t124915 * t34960 * t1294 - F::new(0.1859366460452550541e-3) * t131657 * t8945 * t8948 - F::new(0.17135921299530705785e1) * t124671 * t34940 - F::new(0.17135921299530705785e1) * t131826 * t33471 + F::new(0.11156198762715303246e-2) * t124819 * t31993 * t3719 * t131631 - F::new(0.34694512752820797848e1) * t124626 * t7652 * t5422 - t124927 + F::new(0.34694512752820797848e1) * t124605 * t7637 * t105270 - F::new(0.52041769129231196772e1) * t26948 * t124604 * t7637 * t5230 - F::new(0.34271842599061411569e1) * t33477 * t33478 * t34908 * t1294 + F::new(0.34694512752820797848e1) * t124665 * t29179;
    t131849
}
