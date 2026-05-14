//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1071/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1071<F: Float>(t16897: F, t16721: F, t16775: F, t16779: F, t16886: F, t16889: F, t16893: F, t20329: F, t20330: F, t20331: F, t20333: F, t20335: F, t20337: F, t20338: F, t20339: F, t20341: F, t20342: F) -> (F, F) {
    let t20343 = 3.0 * t16897;
    let t20344 = t20329 - t20330 - t20331 + t20333 - t20335 + t20337 + t20338 - t20339 - t16886 - t16889 - t20341 - t16893 - t20342 + t20343 + t16721 - t16775 - t16779;
    (t20343, t20344)
}
