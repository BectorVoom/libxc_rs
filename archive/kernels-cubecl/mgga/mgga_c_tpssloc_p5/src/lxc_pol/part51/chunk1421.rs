//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1421/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1421<F: Float>(t33363: F, t6880: F, t2018: F, t26161: F, t26558: F, t5356: F, t33273: F, t81159: F, t115545: F, t22633: F, t26215: F, t33272: F, t80650: F) -> (F, F, F, F, F) {
    let t122088 = F::cast_from(3.0_f64) * t33363 * t6880;
    let t122094 = F::cast_from(2.0_f64) * t26161 * t26558 * t2018 * t5356;
    let t122102 = t81159 * t33273;
    let t122107 = t22633 * t115545 * t26215;
    let t122110 = t22633 * t80650 * t33272;
    (t122088, t122094, t122102, t122107, t122110)
}
