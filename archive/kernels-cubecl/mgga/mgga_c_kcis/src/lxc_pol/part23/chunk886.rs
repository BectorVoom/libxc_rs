//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 886/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk886<F: Float>(t1495: F, t16653: F, t1468: F, t1464: F, t2011: F, t3722: F, t4135: F, t1395: F, t3728: F, t5877: F, t1489: F, t5627: F) -> (F, F, F, F, F) {
    let t16654 = t1495 * t16653;
    let t16655 = t1468 * t16654;
    let t16656 = t1464 * t16655;
    let t16658 = t2011 * t3722;
    let t16659 = t4135 * t16658;
    let t16660 = t1395 * t16659;
    let t16661 = t1464 * t16660;
    let t16663 = t3728 * t5877;
    let t16665 = t5627 * t1489;
    (t16656, t16658, t16661, t16663, t16665)
}
