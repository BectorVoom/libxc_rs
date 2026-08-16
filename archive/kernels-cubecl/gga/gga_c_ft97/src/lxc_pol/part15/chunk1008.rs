//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1008/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1008<F: Float>(t378: F, t85474: F, t92: F, t1570: F, t85451: F, t85456: F, t37355: F, t85469: F, t38052: F, t358: F, t85501: F, t38063: F, t45304: F, t59002: F, t59007: F, t73975: F, t73977: F, t73985: F) -> (F, F, F, F, F, F, F, F, F) {
    let t85544 = t92 * t378 * t85474;
    let t85546 = t1570 * t85451;
    let t85548 = t92 * t378 * t85546;
    let t85551 = t92 * t378 * t85456;
    let t85554 = t37355 * t85469;
    let t85556 = t92 * t38052 * t85554;
    let t85558 = t358 * t85501;
    let t85560 = t92 * t378 * t85558;
    let t85567 = -F::cast_from(12.0_f64) * t85544 + F::cast_from(2.0_f64) * t85548 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t85551 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t73985 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t85556 - t85560 / F::cast_from(3.0_f64) + t38063 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t45304 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t59002 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t59007 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t73975 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t73977;
    (t85544, t85546, t85548, t85551, t85554, t85556, t85558, t85560, t85567)
}
