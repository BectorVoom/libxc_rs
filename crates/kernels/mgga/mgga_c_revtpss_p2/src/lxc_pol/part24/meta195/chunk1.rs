//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 925/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk925<F: Float>(t1386: F, t2482: F, t596: F, t1384: F, t235: F) -> (F, F, F, F) {
    let t9976 = t2482 * t1386 * t596;
    let t9989 = t1384 * t1384;
    let t9990 = F::cast_from(1.0_f64) / t9989;
    let t9991 = t9990 * t235;
    (t9976, t9989, t9990, t9991)
}
