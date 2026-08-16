//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 531/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk531(t1378: f64, t6460: f64, t1375: f64, t1843: f64, t5215: f64, t5321: f64, t568: f64, t6362: f64, t6364: f64, t6435: f64, t6440: f64, t1297: f64, t1390: f64, t193: f64, t2486: f64, t3701: f64, t3819: f64, t3821: f64, t3823: f64, t3825: f64, t3832: f64, t3836: f64, t3924: f64, t533: f64, t6324: f64, t6329: f64, t6330: f64, t6347: f64, t6399: f64, t6400: f64) -> (f64, f64, f64) {
    let t6461 = t1378 * t6460;
    let t6463 = 2.0_f64 * t1375 * t6440 - t1375 * t6461 - 2.0_f64 * t1843 * t5215 - 2.0_f64 * t1843 * t5321 + t568 * t6362 + 2.0_f64 * t568 * t6364 + t568 * t6435;
    let t6467 = t1390 * t193 * t533 * t6463 - t193 * t3701 * t533 * t6324 + 3.0_f64 * t1297 * t193 * t6347 + 6.0_f64 * t193 * t3924 * t6330 - t2486 + t3819 + t3821 + t3823 + t3825 - t3832 - t3836 + t6329 - t6399 - t6400;
    (t6461, t6463, t6467)
}
