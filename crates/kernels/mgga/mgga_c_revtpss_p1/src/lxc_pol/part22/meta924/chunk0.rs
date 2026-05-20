//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3146/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3146<F: Float>(t1204: F, t5412: F, t1811: F, t3552: F, t1269: F, t17288: F, t3555: F, t5216: F, t3565: F, t5215: F, t487: F, t3566: F) -> (F, F, F, F, F, F, F, F) {
    let t56503 = t1204 * t5412;
    let t56508 = t3552 * t1811;
    let t56519 = t17288 * t1269;
    let t56570 = t3555 * t5412;
    let t56575 = t5216 * t1269;
    let t56587 = t5215 * t3565;
    let t56588 = t56587 * t487;
    let t56607 = t3566 * t5412;
    (t56503, t56508, t56519, t56570, t56575, t56587, t56588, t56607)
}
