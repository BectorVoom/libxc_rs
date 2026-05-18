//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1201/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1201<F: Float>(t1208: F, t30881: F, t487: F, t1828: F, t8190: F, t7652: F, t1287: F, t1794: F, t29122: F, t2150: F, t30840: F, t473: F) -> (F, F, F, F, F, F) {
    let t30882 = t30881 * t1208;
    let t30883 = t30882 * t487;
    let t30886 = t8190 * t1828;
    let t30887 = t7652 * t30886;
    let t30893 = t29122 * t1794 * t1287;
    let t30899 = t2150 * t473 * t30840;
    (t30882, t30883, t30886, t30887, t30893, t30899)
}
