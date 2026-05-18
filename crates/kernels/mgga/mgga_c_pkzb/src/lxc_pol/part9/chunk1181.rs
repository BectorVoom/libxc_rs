//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1181/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1181<F: Float>(t1692: F, t637: F, t1535: F, t16721: F, t16775: F, t16779: F, t16886: F, t16889: F, t16893: F, t20339: F, t20341: F, t20342: F, t20343: F, t7201: F) -> (F, F) {
    let t20578 = t637 * t1692;
    let t20586 = F::new(9.0) * t1535 * t1692 * t7201 + t16721 - t16775 - t16779 - t16886 - t16889 - t16893 - t20339 - t20341 - t20342 + t20343;
    (t20578, t20586)
}
