//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 649/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk649(t413: f64, t3609: f64, t1260: f64, t286: f64, t1251: f64, t1255: f64, t1264: f64, t3484: f64, t3487: f64, t3490: f64, t3499: f64, t3502: f64, t3505: f64, t3510: f64, t3514: f64, t3517: f64, t3522: f64, t3526: f64, t3534: f64, t423: f64) -> (f64, f64, f64) {
    let t418 = 0.0_f64 < t413;
    let t3611 = piecewise3(t418, t3609, -t3609);
    let t3612 = t1260 * t3611;
    let t3613 = t286 * t3612;
    let t3616 = 11.0_f64 / 216.0_f64 * t3484 * t423 - t3487 / 108.0_f64 - t3490 * t1255 / 108.0_f64 + t3490 * t1264 / 36.0_f64 - t3499 + t3502 / 864.0_f64 - t3505 / 288.0_f64 + t1251 * t3510 / 432.0_f64 - t3514 * t3517 / 288.0_f64 - t1251 * t3522 / 288.0_f64 + t1251 * t3526 / 576.0_f64 + t1251 * t3534 / 96.0_f64 - t1251 * t3613 / 192.0_f64;
    (t3611, t3612, t3616)
}
