//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1013/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1013<F: Float>(t1948: F, t2252: F, t342: F, t142: F, t7763: F, t511: F, t8639: F, t11269: F, t1526: F, t1527: F, t15567: F, t16640: F, t1943: F, t343: F, t72: F, t7745: F, t7765: F, t7789: F, t8766: F, t9007: F, t9041: F, t9045: F, t9078: F, t9084: F) -> F {
    let t41305 = t342 * t2252 * t1948;
    let t41318 = t142 * t7763;
    let t41328 = F::cast_from(5.0_f64) / F::cast_from(54.0_f64) * t342 * t8639 * t511;
    let t41329 = t1526 * t1527 * t9078 / F::cast_from(2.0_f64) - t1526 * t1527 * t8766 * t7765 / F::cast_from(2.0_f64) + t15567 * t16640 * t7789 / F::cast_from(2.0_f64) + t41305 / F::cast_from(6.0_f64) + t9084 - t342 * t343 * t72 * t9007 / F::cast_from(4.0_f64) - t1526 * t1527 * t1943 * t7745 / F::cast_from(12.0_f64) - t1526 * t1527 * t9041 / F::cast_from(4.0_f64) - F::cast_from(7.0_f64) / F::cast_from(27.0_f64) * t1526 * t11269 * t41318 * t7765 - t1526 * t1527 * t9045 / F::cast_from(4.0_f64) - t41328;
    t41329
}
