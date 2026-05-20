//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2611/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2611<F: Float>(t1437: F, t2482: F, t4104: F, t5658: F, t2782: F, t4086: F, t48015: F, t543: F, t1882: F, t3923: F, t4003: F, t10022: F) -> (F, F, F, F) {
    let t48058 = t2482 * t1437 * t5658 * t4104;
    let t48066 = t2782 * t4086 * t48015 * t543;
    let t48073 = t1882 * t3923;
    let t48074 = t48073 * t4003;
    let t48076 = t2782 * t10022 * t48074;
    (t48058, t48066, t48073, t48076)
}
