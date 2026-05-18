//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1266/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1266<F: Float>(t169: F, t34159: F, t5486: F, t619: F, t11361: F, t27658: F, t2993: F, t11601: F, t9291: F, t3691: F, t8965: F, t1030: F, t1971: F, t9267: F, t9272: F) -> (F, F, F, F, F) {
    let t35090 = t169 * t5486 * t34159 * t619;
    let t35093 = t2993 * t11361 * t27658;
    let t35095 = t11601 * t9291;
    let t35097 = t3691 * t8965;
    let t35105 = t1030 * t1971 * t9267 * t9272;
    (t35090, t35093, t35095, t35097, t35105)
}
