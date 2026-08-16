//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 924/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk924(t10309: f64, t248: f64, t3062: f64, t3103: f64, t3114: f64, t376: f64, t676: f64, t1023: f64, t1020: f64, t1041: f64, t10433: f64, t10436: f64, t10438: f64, t10441: f64, t10446: f64, t10449: f64, t10455: f64, t10460: f64, t10463: f64, t10480: f64, t10485: f64, t10490: f64, t10493: f64, t10496: f64, t3039: f64, t3048: f64, t3064: f64, t3098: f64, t3117: f64, t3123: f64, t378: f64) -> (f64, f64, f64) {
    let t10501 = t248 * t3062 * t10309;
    let t10504 = t3114 * t3103;
    let t10508 = t676 * t376;
    let t10510 = t248 * t10508 * t1023;
    let t10511 = t1020 * t10510;
    let t10513 = -t3039 * t10433 / 1024.0_f64 - t10436 / 4608.0_f64 + 19.0_f64 / 576.0_f64 * t10438 * t378 - t10441 / 144.0_f64 - 209.0_f64 / 2592.0_f64 * t10446 * t378 + 19.0_f64 / 864.0_f64 * t10449 - 5.0_f64 / 864.0_f64 * t3048 * t3064 + t10455 / 2304.0_f64 + 5.0_f64 / 6912.0_f64 * t10460 + t1041 * t10463 / 4608.0_f64 + t3114 * t3123 / 1024.0_f64 + t10480 * t10485 / 512.0_f64 - t10490 / 1152.0_f64 + t1041 * t10493 / 768.0_f64 - t10496 / 144.0_f64 - t3117 * t3098 / 768.0_f64 - 5.0_f64 / 2304.0_f64 * t1041 * t10501 + t10504 / 768.0_f64 + t3048 * t3098 / 144.0_f64 - t10511 / 4608.0_f64;
    (t10501, t10510, t10513)
}
