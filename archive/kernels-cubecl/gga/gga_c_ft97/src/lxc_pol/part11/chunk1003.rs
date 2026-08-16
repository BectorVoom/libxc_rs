//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1003/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1003<F: Float>(t2174: F, t8232: F, t1882: F, t9264: F, t2101: F, t2179: F, t9290: F, t12746: F, t13140: F, t144: F, t1901: F, t2178: F, t2185: F, t2210: F, t2221: F, t3434: F, t379: F, t40525: F, t40700: F, t40739: F, t446: F, t609: F, t616: F, t9115: F, t9144: F, t9284: F, t9293: F, t9311: F, t9316: F, t9438: F, t9440: F) -> F {
    let t40900 = t8232 * t2174;
    let t40905 = t1882 * t9264;
    let t40911 = t2101 * t2179;
    let t40916 = t1882 * t9290;
    let t40922 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t2221 * t3434 * t40739 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1901 * t9115 * t12746 * t40700 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t2210 * t9438 * t9440 * t379 - F::cast_from(8.0_f64) * t1901 * t13140 * t2178 * t609 * t9284 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t40900 + F::cast_from(4.0_f64) * t446 * t144 * t40525 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t40905 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t9144 * t9316 * t379 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t40911 * t9311 * t379 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t40916 + F::cast_from(8.0_f64) * t446 * t2185 * t616 * t9293;
    t40922
}
