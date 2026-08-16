//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 317/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk317(t1512: f64, t499: f64, t493: f64, t1286: f64, t381: f64, t498: f64, t1484: f64, t1490: f64, t1494: f64, t1498: f64, t1502: f64, t1507: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1513 = t1512 * t499;
    let t1514 = t493 * t1513;
    let t1516 = t381 * t1286;
    let t1517 = t498 * t1516;
    let t1518 = t493 * t1517;
    let t1520 = t1484 / 16.0_f64 - t1490 / 16.0_f64 - t1494 / 6.0_f64 + t1498 / 24.0_f64 - t1502 / 256.0_f64 + t1507 / 256.0_f64 + t1514 / 48.0_f64 - t1518 / 192.0_f64;
    (t1513, t1514, t1516, t1517, t1518, t1520)
}
