//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1370/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1370(t381: f64, t42348: f64, t23508: f64, t360: f64, t1003: f64, t1022: f64, t10359: f64, t1058: f64, t1060: f64, t1063: f64, t11007: f64, t11027: f64, t11031: f64, t11043: f64, t11065: f64, t11066: f64, t14590: f64, t3180: f64, t3186: f64, t3188: f64, t3189: f64, t3196: f64, t353: f64, t383: f64, t43419: f64, t43480: f64, t43483: f64, t43489: f64, t43503: f64, t4673: f64) -> (f64, f64) {
    let t43504 = t381 * t42348;
    let t43505 = t23508 * t360;
    let t43512 = 4.0_f64 * t1022 * t1058 * t1060 * t11007 + 8.0_f64 * t11027 * t3186 * t4673 - 24.0_f64 * t11065 * t11066 * t43483 - 36.0_f64 * t11065 * t14590 * t3196 + 12.0_f64 * t3186 * t3188 * t43489 + t353 * t383 * t43419 - t43503 * t43504 * t43505 + 4.0_f64 * t1003 * t11043 + 4.0_f64 * t10359 * t1063 + 12.0_f64 * t11031 * t3180 + 12.0_f64 * t3189 * t43480;
    (t43504, t43512)
}
