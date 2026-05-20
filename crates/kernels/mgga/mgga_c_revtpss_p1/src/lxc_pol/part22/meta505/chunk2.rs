//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2247/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2247<F: Float>(t16272: F, t16310: F, t16355: F, t16610: F, t1100: F, t1102: F, t15418: F, t15420: F, t15423: F, t15425: F, t15427: F, t15477: F, t15515: F, t15549: F, t15551: F, t15553: F, t15555: F, t15558: F, t15561: F, t15562: F, t15566: F, t15571: F, t15575: F, t15577: F, t16181: F, t198: F, t3333: F, t336: F, t5023: F) -> (F, F) {
    let t16612 = t16272 + t16310 + t16355 + t16610;
    let t16616 = t1102 * t16612 * t198 * t336 - F::new(2.0) * t1100 * t15562 * t5023 + F::new(2.0) * t15566 * t3333 * t5023 + t15418 + t15420 + t15423 + t15425 + t15427 + t15477 - t15515 - t15549 - t15551 - t15553 - t15555 - t15558 - t15561 + t15571 + t15575 + t15577 - t16181;
    (t16612, t16616)
}
