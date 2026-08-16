//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2078/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2078<F: Float>(t16524: F, t23896: F, t45560: F, t7769: F, t16521: F, t6534: F, t1873: F, t55405: F, t23893: F, t12524: F, t26550: F, t16535: F, t7467: F) -> (F, F, F, F, F, F, F) {
    let t86639 = F::cast_from(27.0_f64) * t16524 * t23896;
    let t86642 = F::cast_from(27.0_f64) * t45560 * t7769;
    let t86646 = F::cast_from(27.0_f64) * t16521 * t6534;
    let t86651 = F::cast_from(27.0_f64) * t55405 * t1873;
    let t86653 = F::cast_from(54.0_f64) * t16524 * t23893;
    let t86655 = F::cast_from(54.0_f64) * t12524 * t26550;
    let t86660 = F::cast_from(27.0_f64) * t16535 * t7467;
    (t86639, t86642, t86646, t86651, t86653, t86655, t86660)
}
