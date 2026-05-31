//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 348/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk348<F: Float>(t1773: F, t1776: F, t1778: F, t1783: F, t1788: F, t1793: F, t1797: F, t1802: F, t1806: F, t462: F, t92: F, t457: F, t91: F) -> (F, F) {
    let t1808 = t1773 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1776 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1778 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t462 * t1783 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t1788 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t1793 - t462 * t1797 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t92 * t1802 - t92 * t1806;
    let t1810 = t91 * t457 * t1808;
    (t1808, t1810)
}
