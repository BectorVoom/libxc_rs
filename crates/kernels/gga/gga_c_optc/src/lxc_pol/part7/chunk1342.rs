//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1342/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1342<F: Float>(t26777: F, t26790: F, t415: F, t26252: F, t26258: F, t26278: F, t26280: F, t26284: F, t26289: F, t26293: F, t26296: F, t26300: F, t26304: F, t26306: F) -> (F, F) {
    let t26792 = (t26777 + t26790) * t415;
    let t26805 = F::cast_from(0.25367901234567901233e-1_f64) * t26252 + F::cast_from(0.2283111111111111111e0_f64) * t26258 - F::cast_from(0.11415555555555555555e0_f64) * t26278 + F::cast_from(0.13698666666666666667e0_f64) * t26280 - F::cast_from(0.41095999999999999999e0_f64) * t26284 + F::cast_from(0.41095999999999999998e0_f64) * t26289 - F::cast_from(0.34246666666666666665e-1_f64) * t26293 + F::cast_from(0.41096e0_f64) * t26296 - F::cast_from(0.61644e0_f64) * t26300 + F::cast_from(0.10274e0_f64) * t26304 - F::cast_from(0.13698666666666666667e0_f64) * t26306;
    (t26792, t26805)
}
