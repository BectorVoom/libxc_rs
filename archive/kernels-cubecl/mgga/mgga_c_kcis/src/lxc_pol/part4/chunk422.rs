//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 422/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk422<F: Float>(t1369: F, t286: F, t531: F, t617: F, t833: F, t616: F) -> (F, F, F, F, F, F) {
    let t1600 = t286 * t1369;
    let t1601 = t617 * t531;
    let t1602 = t1601 * t833;
    let t1603 = t1600 * t1602;
    let t1606 = t616 * t616;
    let t1607 = F::cast_from(1.0_f64) / t1606;
    (t1600, t1601, t1602, t1603, t1606, t1607)
}
