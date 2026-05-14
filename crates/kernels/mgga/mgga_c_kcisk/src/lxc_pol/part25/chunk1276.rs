//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1276/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1276<F: Float>(t47019: F, t9967: F, t33080: F, t65005: F, t2594: F, t33130: F, t5218: F, t17182: F, t34217: F, t9664: F, t17010: F, t1772: F, t648: F, t32989: F, t7218: F, t34200: F, t5074: F) -> (F, F, F, F, F, F, F, F) {
    let t116096 = 2.0 * t47019 * t9967;
    let t116098 = 6.0 * t65005 * t33080;
    let t116101 = 2.0 * t5218 * t33130 * t2594;
    let t116116 = t17182 * t34217;
    let t116118 = 0.69444444444444444446e-2 * t9664 * t116116;
    let t116120 = t17010 * t648 * t1772;
    let t116123 = t32989 * t7218;
    let t116126 = t5074 * t34200;
    (t116096, t116098, t116101, t116116, t116118, t116120, t116123, t116126)
}
