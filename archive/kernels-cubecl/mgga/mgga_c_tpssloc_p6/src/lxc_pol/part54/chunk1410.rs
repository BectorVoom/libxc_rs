//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1410/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1410<F: Float>(t113: F, t121958: F, t122082: F, t33363: F, t6880: F, t2018: F, t26161: F, t26558: F, t5356: F, t33273: F, t81159: F, t115545: F, t22633: F, t26215: F) -> (F, F, F, F, F) {
    let t122084 = t113 * (t121958 + t122082);
    let t122088 = F::cast_from(3.0_f64) * t33363 * t6880;
    let t122094 = F::cast_from(2.0_f64) * t26161 * t26558 * t2018 * t5356;
    let t122102 = t81159 * t33273;
    let t122107 = t22633 * t115545 * t26215;
    (t122084, t122088, t122094, t122102, t122107)
}
