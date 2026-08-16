//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1015/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1015<F: Float>(t1526: F, t7705: F, t8775: F, t15567: F, t16633: F, t3088: F, t41332: F, t41335: F, t41338: F, t41341: F, t41344: F, t41349: F, t7765: F, t7807: F, t8788: F, t8790: F, t9050: F) -> F {
    let t41358 = t1526 * t7705 * t8775;
    let t41360 = F::cast_from(2.0_f64) * t8790 + t41332 / F::cast_from(18.0_f64) - t41335 / F::cast_from(6.0_f64) - t41338 / F::cast_from(12.0_f64) - t41341 / F::cast_from(9.0_f64) + t8788 - t41344 / F::cast_from(4.0_f64) - t1526 * t3088 * t9050 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1526 * t3088 * t41349 * t7765 - t15567 * t16633 * t7807 / F::cast_from(3.0_f64) + t41358 / F::cast_from(6.0_f64);
    t41360
}
