//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1325/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1325<F: Float>(t1333: F, t34174: F, t32948: F, t34045: F, t32921: F, t1772: F, t2447: F, t4830: F, t32947: F, t7218: F, t3805: F, t9957: F, t116474: F, t9649: F, t116350: F, t11245: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t116747 = t1333 * t34174;
    let t116748 = 0.33163888888888888888e-2 * t116747;
    let t116768 = 0.26805555555555555556e-2 * t32948 * t34045;
    let t116771 = 0.26805555555555555556e-2 * t32921 * t34045;
    let t116790 = t4830 * t2447 * t1772;
    let t116793 = t32947 * t7218;
    let t116836 = t3805 * t9957;
    let t116856 = 0.26805555555555555556e-2 * t9649 * t116474;
    let t116866 = t9649 * t116350;
    let t116882 = t11245 * t2447 * t1772;
    (t116747, t116748, t116768, t116771, t116790, t116793, t116836, t116856, t116866, t116882)
}
