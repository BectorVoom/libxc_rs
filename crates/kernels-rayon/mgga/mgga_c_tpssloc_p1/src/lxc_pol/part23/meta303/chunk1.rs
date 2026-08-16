//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1035/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1035(t21592: f64, t21593: f64, t360: f64, t1021: f64, t248: f64, t1044: f64, t21134: f64, t21138: f64, t1020: f64, t1041: f64, t1622: f64, t17607: f64, t18042: f64, t21562: f64, t21566: f64, t21570: f64, t21574: f64, t21580: f64, t3070: f64, t4641: f64, t4644: f64, t5857: f64, t5861: f64, t5869: f64, t5900: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21594 = t21592 + t21593;
    let t21595 = t21594 * t360;
    let t21597 = t248 * t1021 * t21595;
    let t21603 = t248 * t1044 * t21134;
    let t21609 = t248 * t1044 * t21138;
    let t21612 = t973 * t21562 / 48.0_f64 + t3070 * t21566 / 1536.0_f64 + 5.0_f64 / 4608.0_f64 * t3070 * t21570 + t3070 * t21574 / 1536.0_f64 - t4644 * t5900 / 768.0_f64 - 5.0_f64 / 2304.0_f64 * t1041 * t21580 + t18042 / 1152.0_f64 + t17607 * t1622 / 1536.0_f64 + t4641 * t5869 / 1024.0_f64 + t1020 * t21597 / 3072.0_f64 + t4644 * t5857 / 1536.0_f64 + t1041 * t21603 / 4608.0_f64 + 5.0_f64 / 4608.0_f64 * t4644 * t5861 + t1041 * t21609 / 768.0_f64;
    (t21594, t21595, t21597, t21603, t21609, t21612)
}
