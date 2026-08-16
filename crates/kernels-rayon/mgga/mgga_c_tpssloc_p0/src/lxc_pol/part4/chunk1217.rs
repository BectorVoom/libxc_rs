//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1217/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1217(t19755: f64, t20021: f64, t1378: f64, t1385: f64, t6460: f64, t3887: f64, t225: f64, t6364: f64, t20009: f64, t539: f64, t1375: f64, t1386: f64, t16030: f64, t16439: f64, t1843: f64, t19635: f64, t19644: f64, t19648: f64, t3882: f64, t5321: f64, t5326: f64, t5354: f64, t568: f64, t6461: f64) -> f64 {
    let t20022 = t19755 + t20021;
    let t20023 = t1378 * t20022;
    let t20025 = t6460 * t1385;
    let t20026 = t3887 * t20025;
    let t20029 = t6364 * t225;
    let t20032 = t539 * t20009;
    let t20034 = 4.0_f64 * t1375 * t19648 - t1375 * t20023 + 2.0_f64 * t1375 * t20026 - 2.0_f64 * t1386 * t20029 - 2.0_f64 * t16030 * t1843 - 2.0_f64 * t16439 * t1843 + 2.0_f64 * t19635 * t568 + 2.0_f64 * t19644 * t568 + t20032 * t568 - t3882 * t6461 + 4.0_f64 * t5321 * t5326 - 2.0_f64 * t5321 * t5354;
    t20034
}
