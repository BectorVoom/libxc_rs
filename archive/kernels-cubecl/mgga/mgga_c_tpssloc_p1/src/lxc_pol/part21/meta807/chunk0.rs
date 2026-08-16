//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2810/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2810<F: Float>(t2627: F, t5631: F, t13176: F, t13417: F, t13431: F, t13434: F, t1523: F, t16823: F, t17028: F, t255: F, t2617: F, t2633: F, t4162: F, t4166: F, t4296: F, t4298: F, t46528: F, t5648: F, t5653: F, t59074: F, t59230: F, t812: F, t860: F, t9612: F) -> F {
    let t59355 = t2627 * t5631;
    let t59379 = F::cast_from(2.0_f64) * t2633 * t59355 * t812 - t59074 * t812 * t860 - F::cast_from(4.0_f64) * t13176 * t4296 + F::cast_from(4.0_f64) * t13417 * t4166 - F::cast_from(2.0_f64) * t13431 * t4166 - F::cast_from(4.0_f64) * t13434 * t4166 - F::cast_from(2.0_f64) * t1523 * t46528 - F::cast_from(2.0_f64) * t16823 * t2617 - F::cast_from(2.0_f64) * t17028 * t2617 + t255 * t59230 + F::cast_from(4.0_f64) * t4162 * t4298 - F::cast_from(2.0_f64) * t5648 * t9612 - t5653 * t9612;
    t59379
}
