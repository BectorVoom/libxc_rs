//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3906/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3906<F: Float>(t1883: F, t5658: F, t2782: F, t4100: F, t543: F, t73842: F, t22331: F, t2470: F, t4101: F, t48048: F, t5741: F, t10073: F, t22369: F) -> (F, F, F, F, F) {
    let t75012 = t1883 * t5658;
    let t75014 = t2782 * t4100 * t75012;
    let t75016 = t73842 * t543;
    let t75018 = t2782 * t4100 * t75016;
    let t75021 = t4101 * t22331 * t2470;
    let t75024 = t48048 * t5741;
    let t75026 = t10073 * t22369;
    (t75014, t75018, t75021, t75024, t75026)
}
