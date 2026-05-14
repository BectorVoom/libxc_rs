//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1082/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1082<F: Float>(t20334: F, t24642: F, t20340: F, t16721: F, t16775: F, t16779: F, t16783: F, t16787: F, t16875: F, t16886: F, t16889: F, t16893: F, t16897: F, t19825: F, t20337: F, t20338: F, t20339: F) -> (F, F, F, F) {
    let t29126 = 36.0 * t20334;
    let t29127 = 0.73245789224026180216e-3 * t24642;
    let t29128 = 0.17544670867903938621e1 * t20340;
    let t29129 = -t19825 - t16875 - t29126 - t20337 - t20338 + t20339 + t29127 - t16886 - t16889 - t29128 - t16893 + t16897 + t16721 - t16775 - t16779 + t16783 - t16787;
    (t29126, t29127, t29128, t29129)
}
