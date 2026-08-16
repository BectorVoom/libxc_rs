//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 650/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk650<F: Float>(t1964: F, t7765: F, t356: F, t89: F, t569: F, t7789: F, t446: F, t8796: F, t8799: F, t8802: F, t8805: F, t9010: F, t9014: F, t9020: F, t9024: F, t9028: F) -> (F, F, F, F, F) {
    let t9030 = t1964 * t7765;
    let t9032 = t89 * t356 * t9030;
    let t9034 = t569 * t7789;
    let t9035 = t446 * t9034;
    let t9037 = -F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8796 + t8799 / F::cast_from(18.0_f64) + t8802 / F::cast_from(27.0_f64) - t8805 / F::cast_from(3.0_f64) - t9010 / F::cast_from(6.0_f64) - t9014 / F::cast_from(18.0_f64) - t9020 + t9024 - F::cast_from(5.0_f64) / F::cast_from(81.0_f64) * t9028 - t9032 / F::cast_from(3.0_f64) + t9035 / F::cast_from(3.0_f64);
    (t9030, t9032, t9034, t9035, t9037)
}
