//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1310/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1310<F: Float>(t62249: F, t9664: F, t9671: F, t18325: F, t32989: F, t4597: F, t4826: F, t33001: F, t11197: F, t1763: F, t1772: F, t46928: F, t648: F, t9651: F, t10473: F, t9681: F) -> (F, F, F, F, F, F, F, F, F) {
    let t112283 = t9664 * t62249 * t9671;
    let t112289 = t32989 * t18325;
    let t112372 = t4826 * t4597;
    let t112406 = t33001 * t18325;
    let t112416 = t11197 * t1763 * t1772;
    let t112451 = t46928 * t648 * t1772;
    let t112517 = t62249 * t9651;
    let t112518 = t9664 * t112517;
    let t112523 = t10473 * t9681;
    (t112283, t112289, t112372, t112406, t112416, t112451, t112517, t112518, t112523)
}
