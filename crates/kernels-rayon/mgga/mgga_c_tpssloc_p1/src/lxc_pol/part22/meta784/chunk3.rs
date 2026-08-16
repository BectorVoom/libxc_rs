//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2694/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2694(t1338: f64, t20601: f64, t1336: f64, t1352: f64, t16040: f64, t16060: f64, t16132: f64, t1825: f64, t19668: f64, t19732: f64, t20473: f64, t20625: f64, t20643: f64, t3777: f64, t5234: f64, t5334: f64, t5348: f64, t5351: f64, t57659: f64, t6378: f64, t6415: f64, t6448: f64, t6451: f64) -> f64 {
    let t75124 = t1338 * t20601;
    let t75150 = -t1336 * t1352 * t75124 - 3.0_f64 * t1336 * t16132 * t6415 - 3.0_f64 * t1336 * t1825 * t57659 - 3.0_f64 * t1336 * t19732 * t5348 + 6.0_f64 * t16040 * t20473 * t5334 + 6.0_f64 * t16060 * t6448 - 6.0_f64 * t16060 * t6451 + 6.0_f64 * t19668 * t5234 + 6.0_f64 * t20625 * t3777 - t20643 * t3777 + 3.0_f64 * t5351 * t6378;
    t75150
}
