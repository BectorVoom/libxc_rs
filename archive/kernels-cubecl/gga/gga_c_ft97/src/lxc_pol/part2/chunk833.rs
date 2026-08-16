//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 833/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk833<F: Float>(t1882: F, t3467: F, t12606: F, t144: F, t1053: F, t1986: F, t2185: F, t605: F, t12306: F, t12308: F, t12310: F, t12285: F, t12290: F, t12293: F, t12296: F, t12300: F, t12304: F, t12315: F, t12881: F) -> (F, F, F, F) {
    let t13084 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1882 * t3467;
    let t13085 = t144 * t12606;
    let t13088 = t1053 * t1986;
    let t13090 = t2185 * t605 * t13088;
    let t13100 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12306;
    let t13101 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t12308;
    let t13102 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t12310;
    let t13104 = t12881 / F::cast_from(2.0_f64) + t12285 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12290 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t12293 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t12296 + t12300 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12304 - t13100 - t13101 + t13102 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t12315;
    (t13084, t13085, t13090, t13104)
}
