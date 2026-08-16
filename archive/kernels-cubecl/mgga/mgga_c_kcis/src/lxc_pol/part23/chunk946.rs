//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 946/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk946<F: Float>(t17453: F, t5904: F, t4292: F, t16653: F, t4293: F, t15898: F, t4261: F, t4260: F, t11825: F, t4291: F, t15973: F, t6011: F) -> (F, F, F, F, F, F, F, F) {
    let t17454 = t5904 * t17453;
    let t17455 = t4292 * t17454;
    let t17457 = t4293 * t16653;
    let t17458 = t4292 * t17457;
    let t17460 = t4261 * t15898;
    let t17461 = t4260 * t17460;
    let t17463 = t11825 * t4291;
    let t17464 = t6011 * t15973;
    (t17454, t17455, t17457, t17458, t17460, t17461, t17463, t17464)
}
