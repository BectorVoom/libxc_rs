//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 466/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk466(t1248: f64, t990: f64, t806: f64, t298: f64, t35: f64, t1216: f64, t1000: f64, t1256: f64, t810: f64, t308: f64, t1268: f64, t295: f64, t305: f64, t803: f64, t811: f64, t991: f64, t997: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2358 = t1248 * t990;
    let t2359 = t2358 * t806;
    let t2362 = t298 * t35;
    let t2363 = t2362 * t1216;
    let t2368 = t1256 * t1000;
    let t2369 = t2368 * t810;
    let t2372 = t308 * t35;
    let t2373 = t2372 * t1216;
    let t2376 = -25.0_f64 / 9.0_f64 * t803 * t991 + 10.0_f64 / 9.0_f64 * t295 * t2359 + 5.0_f64 / 3.0_f64 * t295 * t2363 - 25.0_f64 / 9.0_f64 * t997 * t811 + 10.0_f64 / 9.0_f64 * t305 * t2369 - 5.0_f64 / 3.0_f64 * t305 * t2373 - t1268;
    (t2358, t2359, t2363, t2368, t2369, t2373, t2376)
}
