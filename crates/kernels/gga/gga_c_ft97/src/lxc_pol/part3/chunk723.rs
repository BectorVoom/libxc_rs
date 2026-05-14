//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 723/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk723<F: Float>(t1546: F, t4660: F, t89: F, t4652: F, t7780: F, t1984: F, t4714: F, t558: F, t28: F, t15625: F, t519: F, t356: F, t1013: F, t554: F, t12401: F, t4710: F, t549: F) -> (F, F, F, F, F, F) {
    let t16748 = t89 * t1546 * t4660;
    let t16751 = t89 * t7780 * t4652;
    let t16753 = t1984 * t4714;
    let t16754 = t16753 * t558;
    let t16756 = t89 * t28 * t16754;
    let t16758 = t519 * t15625;
    let t16760 = t89 * t356 * t16758;
    let t16762 = t1013 * t554;
    let t16763 = t12401 * t16762;
    let t16769 = t549 * t4710;
    (t16748, t16751, t16756, t16760, t16763, t16769)
}
