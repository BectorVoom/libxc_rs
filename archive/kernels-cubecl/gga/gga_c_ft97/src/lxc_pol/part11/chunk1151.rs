//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1151/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1151<F: Float>(t2349: F, t2844: F, t875: F, t9571: F, t2803: F, t8232: F, t10488: F, t8392: F, t2739: F, t10479: F, t10485: F, t15182: F, t1901: F, t1934: F, t2857: F, t2881: F, t296: F, t319: F, t4140: F, t41726: F, t43948: F, t44190: F, t44195: F, t44202: F, t44204: F, t446: F) -> (F, F, F, F) {
    let t44205 = t2349 * t2844;
    let t44210 = t9571 * t875;
    let t44215 = t8232 * t2803;
    let t44217 = t8392 * t10488;
    let t44219 = t2349 * t2739;
    let t44224 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t2857 * t319 * t41726 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t44190 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t296 * t43948 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t44195 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t2881 * t15182 * t1934 * t2844 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t44202 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t2881 * t44204 * t44205 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t2881 * t10485 * t44210 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t44215 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t44217 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t10479 * t4140 * t44219;
    (t44205, t44210, t44219, t44224)
}
