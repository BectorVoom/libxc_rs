//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1076/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1076(t24567: f64, t8871: f64, t225: f64, t497: f64, t7348: f64, t462: f64, t1238: f64, t1252: f64, t2121: f64, t32422: f64, t32452: f64, t32480: f64, t32482: f64, t32489: f64, t32493: f64, t32498: f64, t3487: f64, t3593: f64, t498: f64, t7283: f64, t7351: f64, t7356: f64, t8888: f64, t8898: f64) -> (f64, f64, f64, f64) {
    let t32499 = t24567 * t8871;
    let t32503 = t7348 * t225 * t497;
    let t32504 = t462 * t32503;
    let t32507 = 4.0_f64 * t7351 * t7356 + t32422 * t498 + t32452 * t498 - t1238 * t32480 - t32482 * t1252 - t3593 * t8898 - t3487 * t8898 + 2.0_f64 * t3593 * t8888 + 2.0_f64 * t1238 * t32489 + 4.0_f64 * t1238 * t32493 + t32498 - 0.16449340668482264365e-1_f64 * t7283 * t32499 + 0.16449340668482264365e-1_f64 * t2121 * t32504;
    (t32499, t32503, t32504, t32507)
}
