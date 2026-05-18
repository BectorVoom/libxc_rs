//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1196/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1196<F: Float>(t7968: F, t95006: F, t12651: F, t1616: F, t12581: F, t1598: F, t251: F, t1370: F, t27614: F, t27664: F, t4425: F, t7978: F) -> (F, F, F, F, F) {
    let t95007 = t7968 * t95006;
    let t95009 = t12651 * t1616;
    let t95021 = t12581 * t251 * t1598;
    let t95024 = t1370 * t27614;
    let t95042 = t7978 * t4425 * t27664;
    (t95007, t95009, t95021, t95024, t95042)
}
