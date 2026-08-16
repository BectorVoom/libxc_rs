//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1977/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1977(t225: f64, t29071: f64, t1510: f64, t24269: f64, t2617: f64, t26598: f64, t28997: f64, t29052: f64, t4166: f64, t5612: f64, t5617: f64, t812: f64, t87068: f64, t92491: f64, t92546: f64, t98325: f64, t98328: f64, t98330: f64, t98334: f64, t98339: f64, t98342: f64, t98345: f64, t98349: f64, t98353: f64) -> (f64, f64) {
    let t101593 = t29071 * t225;
    let t101618 = 0.6579736267392905746e-1_f64 * t98325 - 2.0_f64 * t4166 * t26598 - 0.19739208802178717238e0_f64 * t98328 - 0.23029076935875170111e0_f64 * t98330 - 2.0_f64 * t812 * t92546 * t1510 - t812 * t24269 * t5617 + 0.16449340668482264365e-1_f64 * t98334 - t812 * t24269 * t5612 - 0.9869604401089358619e-1_f64 * t98339 - t2617 * t29052 + t92491 - 0.10417915756705434098e0_f64 * t87068 - 2.0_f64 * t2617 * t28997 - 0.82246703342411321825e-2_f64 * t98342 + 0.3289868133696452873e-1_f64 * t98345 - 0.3289868133696452873e-1_f64 * t98349 - 0.3289868133696452873e-1_f64 * t98353;
    (t101593, t101618)
}
