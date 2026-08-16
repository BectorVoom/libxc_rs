//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1767/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1767<F: Float>(t4351: F, t892: F, t914: F, t2837: F, t4354: F, t1543: F, t2841: F, t2845: F, t10650: F, t1557: F, t2787: F, t4396: F) -> (F, F, F, F, F, F, F) {
    let t13515 = t4351 * t892;
    let t13517 = F::cast_from(2.0_f64) * t13515 * t914;
    let t13519 = F::cast_from(1.0_f64) * t4354 * t2837;
    let t13520 = t1543 * t2841;
    let t13522 = F::cast_from(0.16081979498692535067e2_f64) * t13520 * t2845;
    let t13524 = F::cast_from(1.0_f64) * t10650 * t1557;
    let t13526 = F::cast_from(2.0_f64) * t2787 * t4396;
    (t13515, t13517, t13519, t13520, t13522, t13524, t13526)
}
