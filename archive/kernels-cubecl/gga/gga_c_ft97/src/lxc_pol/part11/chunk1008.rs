//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1008/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1008<F: Float>(t597: F, t9114: F, t12982: F, t13212: F, t144: F, t1651: F, t1901: F, t1986: F, t2179: F, t2180: F, t2185: F, t2190: F, t39646: F, t39660: F, t41084: F, t41093: F, t446: F, t558: F, t574: F, t9099: F, t9117: F, t9123: F, t9144: F, t9349: F, t9354: F, t9419: F, t9439: F, t9440: F) -> F {
    let t41107 = t9114 * t597;
    let t41117 = F::cast_from(8.0_f64) * t446 * t574 * t9439 * t9440 * t558 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t41084 + F::cast_from(8.0_f64) * t446 * t2185 * t2179 * t1986 * t2180 + t41093 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t9144 * t1651 * t2190 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t13212 * t39660 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t446 * t144 * t39646 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t12982 * t9123 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t41107 * t9117 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t9419 * t9349 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t9099 * t9354;
    t41117
}
