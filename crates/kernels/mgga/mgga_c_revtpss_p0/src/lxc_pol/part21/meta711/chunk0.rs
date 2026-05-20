//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2542/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2542<F: Float>(t3981: F, t46644: F, t1412: F, t3889: F, t808: F, t9736: F, t1369: F, t9726: F, t1372: F, t13999: F, t9837: F, t546: F, t9801: F) -> (F, F, F, F, F, F) {
    let t46645 = t46644 * t3981;
    let t46649 = t9736 * t808 * t1412 * t3889;
    let t46651 = t9726 * t1369;
    let t46652 = t46651 * t1372;
    let t46660 = t13999 * t9837;
    let t46670 = t9801 * t546;
    (t46645, t46649, t46651, t46652, t46660, t46670)
}
