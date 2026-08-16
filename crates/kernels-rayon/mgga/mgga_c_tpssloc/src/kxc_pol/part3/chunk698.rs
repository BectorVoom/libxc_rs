//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 698/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk698(t3584: f64, t61: f64, t248: f64, t3243: f64, t1174: f64, t1213: f64, t1218: f64, t1227: f64, t1232: f64, t3490: f64, t3496: f64, t3506: f64, t3511: f64, t3515: f64, t3518: f64, t3524: f64, t3527: f64, t3531: f64, t3536: f64, t3542: f64, t3543: f64, t3547: f64, t3549: f64, t3552: f64, t3557: f64, t3562: f64, t3567: f64, t3573: f64, t3577: f64, t3580: f64, t488: f64) -> (f64, f64, f64) {
    let t3585 = t61 * t3584;
    let t3587 = t248 * t3585 * t3243;
    let t3590 = -t3490 * t1232 / 2304.0_f64 + t1213 * t3496 / 3072.0_f64 + t3506 * t3511 / 1536.0_f64 - t3515 * t3518 / 3072.0_f64 - t3524 / 3456.0_f64 - t1227 * t3527 / 4608.0_f64 - t1227 * t3531 / 2304.0_f64 + t3536 * t1218 / 1536.0_f64 - t3542 + t3543 / 2304.0_f64 - t3547 - t3549 / 432.0_f64 - t1174 * t3552 / 288.0_f64 - t1174 * t3557 / 144.0_f64 + t1174 * t3562 / 216.0_f64 + t3567 * t488 / 3072.0_f64 + t3573 / 2304.0_f64 - t3577 * t3580 / 2304.0_f64 + 5.0_f64 / 13824.0_f64 * t1227 * t3587;
    (t3585, t3587, t3590)
}
