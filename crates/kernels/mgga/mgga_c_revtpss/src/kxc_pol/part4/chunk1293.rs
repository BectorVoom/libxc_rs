//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1293/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1293<F: Float>(t3022: F, t4729: F, t15399: F, t15418: F, t15420: F, t15423: F, t15425: F, t15427: F, t15477: F, t15515: F, t15549: F, t15551: F, t15553: F, t15555: F, t15558: F, t15561: F, t15571: F, t15575: F, t15577: F) -> (F, F) {
    let t16181 = F::cast_from(0.11696447245269292414e1_f64) * t3022 * t4729;
    let t16182 = t15399 + t15418 + t15420 + t15423 + t15425 + t15427 + t15477 - t15549 - t15551 - t15553 - t15555 - t15558 - t15561 - t15515 + t15571 + t15575 + t15577 - t16181;
    (t16181, t16182)
}
