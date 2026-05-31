//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 787/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk787<F: Float>(t12331: F, t446: F, t2223: F, t3337: F, t9073: F, t2992: F, t1969: F, t9065: F, t12285: F, t12290: F, t12293: F, t12296: F, t12300: F, t12304: F, t12307: F, t12309: F, t12311: F, t12315: F, t12319: F, t12322: F, t12325: F, t12328: F, t8805: F, t9068: F) -> (F, F, F, F, F, F) {
    let t12332 = t446 * t12331;
    let t12334 = t3337 * t2223;
    let t12335 = t9073 * t12334;
    let t12336 = t446 * t12335;
    let t12338 = t2992 * t2223;
    let t12339 = t1969 * t12338;
    let t12340 = t446 * t12339;
    let t12343 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t9065;
    let t12345 = t12285 / F::cast_from(18.0_f64) + t12290 / F::cast_from(27.0_f64) - F::cast_from(5.0_f64) / F::cast_from(81.0_f64) * t12293 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t12296 + t12300 / F::cast_from(18.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12304 - t12307 - t12309 + t12311 - t12315 / F::cast_from(9.0_f64) - t12319 / F::cast_from(9.0_f64) - t12322 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12325 - t12328 + t12332 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12336 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12340 - t8805 / F::cast_from(9.0_f64) - t12343 + t9068 / F::cast_from(18.0_f64);
    (t12332, t12334, t12336, t12338, t12340, t12345)
}
