//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 787/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk787(t12331: f64, t446: f64, t2223: f64, t3337: f64, t9073: f64, t2992: f64, t1969: f64, t9065: f64, t12285: f64, t12290: f64, t12293: f64, t12296: f64, t12300: f64, t12304: f64, t12307: f64, t12309: f64, t12311: f64, t12315: f64, t12319: f64, t12322: f64, t12325: f64, t12328: f64, t8805: f64, t9068: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12332 = t446 * t12331;
    let t12334 = t3337 * t2223;
    let t12335 = t9073 * t12334;
    let t12336 = t446 * t12335;
    let t12338 = t2992 * t2223;
    let t12339 = t1969 * t12338;
    let t12340 = t446 * t12339;
    let t12343 = 4.0_f64 / 27.0_f64 * t9065;
    let t12345 = t12285 / 18.0_f64 + t12290 / 27.0_f64 - 5.0_f64 / 81.0_f64 * t12293 - 4.0_f64 / 27.0_f64 * t12296 + t12300 / 18.0_f64 + 2.0_f64 / 9.0_f64 * t12304 - t12307 - t12309 + t12311 - t12315 / 9.0_f64 - t12319 / 9.0_f64 - t12322 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t12325 - t12328 + t12332 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t12336 - 2.0_f64 / 9.0_f64 * t12340 - t8805 / 9.0_f64 - t12343 + t9068 / 18.0_f64;
    (t12332, t12334, t12336, t12338, t12340, t12345)
}
