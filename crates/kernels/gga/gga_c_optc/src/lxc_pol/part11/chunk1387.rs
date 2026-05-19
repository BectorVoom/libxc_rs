//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1387/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1387<F: Float>(t58740: F, t58752: F, t1038: F, t52446: F, t52452: F, t52591: F, t52593: F, t52596: F, t52601: F, t58415: F, t58418: F, t58421: F, t58424: F, t58428: F, t58431: F, t58435: F) -> (F, F, F) {
    let t58753 = t58740 + t58752;
    let t58754 = t1038 * t58753;
    let t58756 = -F::new(0.82785e-1) * t58415 - F::new(0.99342e0) * t58418 - F::new(0.82785e-1) * t58421 + F::new(0.198684e1) * t58424 - F::cast_from(0.8585111111111111111e-1_f64) * t58428 - F::cast_from(0.89459259259259259259e0_f64) * t58431 - F::new(0.301925e0) * t58435 + F::cast_from(0.98115555555555555555e-1_f64) * t52591 - F::new(0.44152e0) * t52593 + F::new(0.132456e1) * t52596 + F::new(0.22076e0) * t52601 + F::cast_from(0.80513333333333333333e0_f64) * t52446 - F::new(0.24154e1) * t52452 + F::new(0.258925e1) * t58754;
    (t58753, t58754, t58756)
}
