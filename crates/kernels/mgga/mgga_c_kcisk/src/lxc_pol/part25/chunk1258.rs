//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1258/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1258<F: Float>(t1693: F, t1772: F, t4982: F, t17182: F, t32937: F, t9664: F, t18325: F, t33001: F, t11197: F, t1763: F, t4823: F, t32912: F, t32909: F, t32921: F, t46928: F, t648: F) -> (F, F, F, F, F, F, F, F, F) {
    let t112387 = t1693 * t4982 * t1772;
    let t112395 = t17182 * t32937;
    let t112396 = t9664 * t112395;
    let t112406 = t33001 * t18325;
    let t112416 = t11197 * t1763 * t1772;
    let t112420 = t4823 * t4982 * t1772;
    let t112439 = t9664 * t17182 * t32912;
    let t112445 = t32921 * t32909;
    let t112451 = t46928 * t648 * t1772;
    (t112387, t112395, t112396, t112406, t112416, t112420, t112439, t112445, t112451)
}
