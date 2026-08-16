//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 982/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk982<F: Float>(t1775: F, t9221: F, t9214: F, t2103: F, t8282: F, t1554: F, t1984: F, t2: F, t9181: F, t11176: F, t151: F, t11761: F, t12791: F, t12823: F, t2102: F, t3499: F, t3506: F, t37264: F, t37269: F, t37311: F, t39694: F, t39698: F, t39713: F, t39730: F, t39739: F, t40323: F, t462: F, t9192: F, t9217: F) -> (F, F) {
    let t40447 = t1775 * t9221;
    let t40449 = t1775 * t9214;
    let t40451 = t8282 * t2103;
    let t40465 = t1554 * t1984;
    let t40466 = t40465 * t2;
    let t40476 = t1775 * t9181;
    let t40485 = F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t11176 * t151;
    let t40486 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t40447 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t40449 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t40451 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t462 * t2102 * t39739 + F::cast_from(2.0_f64) * t462 * t2102 * t39730 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t462 * t3499 * t37269 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t462 * t12823 * t37311 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t462 * t40466 * t39694 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t462 * t9192 * t39698 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t462 * t3506 * t37264 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t40476 - F::cast_from(4.0_f64) * t462 * t9217 * t39713 - F::cast_from(8.0_f64) * t11761 * t12791 * t40323 + t40485;
    (t40465, t40486)
}
