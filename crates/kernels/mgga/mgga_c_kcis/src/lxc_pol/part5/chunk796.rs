//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 796/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk796<F: Float>(t1495: F, t6917: F, t1468: F, t1464: F, t2011: F) -> (F, F, F, F) {
    let t6918 = t1495 * t6917;
    let t6919 = t1468 * t6918;
    let t6920 = t1464 * t6919;
    let t6922 = t2011 * t2011;
    (t6918, t6919, t6920, t6922)
}
