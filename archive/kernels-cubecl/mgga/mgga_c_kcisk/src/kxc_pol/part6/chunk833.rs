//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 833/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk833<F: Float>(t1580: F, t27777: F, t4419: F, t8399: F, t535: F, t8336: F, t2318: F, t6497: F, t3973: F, t8331: F, t1576: F, t8308: F) -> (F, F, F, F, F, F) {
    let t27778 = t1580 * t27777;
    let t27790 = t4419 * t8399;
    let t27791 = t535 * t27790;
    let t27795 = t4419 * t8336;
    let t27796 = t535 * t27795;
    let t27810 = t2318 * t6497;
    let t27861 = t3973 * t8331;
    let t27862 = t1580 * t27861;
    let t27915 = t8308 * t1576;
    (t27778, t27791, t27796, t27810, t27862, t27915)
}
