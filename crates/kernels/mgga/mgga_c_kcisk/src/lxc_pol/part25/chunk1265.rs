//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1265/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1265<F: Float>(t112750: F, t2804: F, t33257: F, t9732: F, t33283: F, t9724: F, t33287: F, t9736: F, t112762: F, t9740: F, t48363: F, t79: F, t33276: F, t9733: F, t25: F, t33218: F) -> (F, F, F, F, F, F, F, F) {
    let t112791 = t2804 * t112750;
    let t112807 = t33257 * t9732;
    let t112810 = t9724 * t33283;
    let t112815 = t33287 * t9736;
    let t112817 = t9740 * t112762;
    let t112835 = t48363 * t79;
    let t112856 = t9733 * t33276;
    let t112858 = t25 * t33218;
    (t112791, t112807, t112810, t112815, t112817, t112835, t112856, t112858)
}
