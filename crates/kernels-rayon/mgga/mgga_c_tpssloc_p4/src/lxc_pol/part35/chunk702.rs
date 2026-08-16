//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 702/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk702(t1297: f64, t1390: f64, t193: f64, t2486: f64, t3701: f64, t3819: f64, t3821: f64, t3823: f64, t3825: f64, t3832: f64, t3836: f64, t3924: f64, t533: f64, t6324: f64, t6329: f64, t6330: f64, t6347: f64, t6399: f64, t6400: f64, t6463: f64) -> f64 {
    let t6467 = t1390 * t193 * t533 * t6463 - t193 * t3701 * t533 * t6324 + 3.0_f64 * t1297 * t193 * t6347 + 6.0_f64 * t193 * t3924 * t6330 - t2486 + t3819 + t3821 + t3823 + t3825 - t3832 - t3836 + t6329 - t6399 - t6400;
    t6467
}
