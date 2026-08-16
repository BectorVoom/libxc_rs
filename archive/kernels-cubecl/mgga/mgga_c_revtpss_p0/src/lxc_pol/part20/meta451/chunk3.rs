//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1720/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1720<F: Float>(t125: F, t9898: F, t13999: F, t9837: F, t546: F, t9801: F, t9738: F, t124: F, t3938: F, t4056: F, t9816: F, t9818: F) -> (F, F, F, F) {
    let t46655 = t125 * t9898;
    let t46660 = t13999 * t9837;
    let t46670 = t9801 * t546;
    let t46671 = t46670 * t9738;
    let t46680 = t9816 * t9818 * t124 * t4056 * t3938;
    (t46655, t46660, t46671, t46680)
}
