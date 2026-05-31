//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 870/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk870<F: Float>(t13567: F, t26: F, t2999: F, t13538: F, t13541: F, t13543: F, t13544: F, t13547: F, t13550: F, t13553: F, t13556: F, t13559: F, t13562: F, t13565: F, t9557: F, t9558: F, t9560: F, t9562: F, t9564: F) -> (F, F) {
    let t13569 = t26 * t2999 * t13567;
    let t13571 = -t9557 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9558 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t9560 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9562 + t9564 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t13538 + t13541 - t13543 - F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t13544 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t13547 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13550 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t13553 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13556 - F::cast_from(2.0_f64) * t13559 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t13562 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t13565 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t13569;
    (t13569, t13571)
}
