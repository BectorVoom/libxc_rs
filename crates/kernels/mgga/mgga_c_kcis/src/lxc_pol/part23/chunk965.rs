//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 965/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk965<F: Float>(t1616: F, t531: F, t18119: F, t4440: F, t4425: F, t7980: F, t7978: F, t12858: F, t251: F, t1598: F) -> (F, F, F, F, F, F, F) {
    let t27584 = t1616 * t531;
    let t27585 = t27584 * t18119;
    let t27586 = t4440 * t27585;
    let t27591 = t4425 * t7980;
    let t27592 = t7978 * t27591;
    let t27594 = t12858 * t251;
    let t27595 = t27594 * t1598;
    (t27584, t27585, t27586, t27591, t27592, t27594, t27595)
}
