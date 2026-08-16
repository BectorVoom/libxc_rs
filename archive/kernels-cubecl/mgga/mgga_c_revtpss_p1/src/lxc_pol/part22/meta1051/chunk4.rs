//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3709/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3709<F: Float>(t1235: F, t371: F, t6645: F, t676: F, t21063: F, t3678: F, t17307: F, t1803: F, t17225: F, t5381: F, t1261: F, t20791: F, t3172: F) -> (F, F, F, F, F) {
    let t70263 = t1235 * t371 * t676 * t6645;
    let t70265 = t21063 * t3678;
    let t70267 = t17307 * t1803;
    let t70270 = t5381 * t17225;
    let t70273 = t1261 * t3172 * t20791;
    (t70263, t70265, t70267, t70270, t70273)
}
