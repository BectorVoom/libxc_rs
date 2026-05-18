//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1254/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1254<F: Float>(t15559: F, t981: F, t3336: F, t5019: F, t11108: F, t1699: F, t3022: F, t4725: F, t11465: F, t1633: F, t3015: F, t3026: F, t4719: F) -> (F, F, F, F, F, F) {
    let t15561 = F::new(0.35089341735807877242e1) * t981 * t15559;
    let t15562 = t5019 * t3336;
    let t15566 = t1699 * t11108;
    let t15571 = F::new(0.23392894490538584828e1) * t3022 * t4725;
    let t15572 = t11465 * t1633;
    let t15573 = t15572 * t3015;
    let t15575 = F::new(0.10389515463408878255e3) * t981 * t15573;
    let t15577 = F::new(0.11696447245269292414e1) * t4719 * t3026;
    (t15561, t15562, t15566, t15571, t15575, t15577)
}
