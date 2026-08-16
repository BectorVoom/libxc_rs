//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1196/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1196<F: Float>(t112943: F, t6562: F, t7488: F, t10110: F, t112863: F, t118802: F, t118810: F, t118814: F, t118825: F, t118828: F, t13042: F, t13053: F, t1492: F, t1912: F, t23278: F, t25184: F, t259: F, t2597: F, t30725: F, t32804: F, t4300: F, t6627: F, t7517: F, t8352: F, t8353: F, t8363: F, t855: F, t86988: F) -> F {
    let t118830 = t6562 * t112943 * t7488;
    let t118831 = F::cast_from(0.82246703342411321825e-2_f64) * t118830;
    let t118832 = -F::cast_from(6.0_f64) * t10110 * t4300 * t8352 * t855 + t1492 * t259 * t30725 + F::cast_from(2.0_f64) * t13042 * t8353 - t13053 * t8363 - F::cast_from(2.0_f64) * t1912 * t86988 + F::cast_from(4.0_f64) * t23278 * t7517 + F::cast_from(4.0_f64) * t25184 * t6627 + F::cast_from(2.0_f64) * t2597 * t32804 + t112863 + t118802 - t118810 + t118814 + t118825 + t118828 + t118831;
    t118832
}
