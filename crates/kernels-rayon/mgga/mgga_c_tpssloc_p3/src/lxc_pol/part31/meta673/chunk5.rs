//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2029/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2029(t1338: f64, t29286: f64, t2085: f64, t6387: f64, t1336: f64, t1352: f64, t16047: f64, t16060: f64, t19744: f64, t19815: f64, t27097: f64, t27103: f64, t29339: f64, t29345: f64, t3777: f64, t5234: f64, t5250: f64, t5287: f64, t5334: f64, t5344: f64, t6388: f64, t7209: f64, t7932: f64, t84577: f64, t91078: f64, t91081: f64, t93792: f64, t93794: f64, t97488: f64, t97491: f64, t97494: f64) -> f64 {
    let t102798 = t1338 * t29286;
    let t102801 = t2085 * t6387;
    let t102822 = 2.0_f64 * t1336 * t84577 * t6388 - 2.0_f64 * t16060 * t7932 + t93792 - 2.0_f64 * t5234 * t27103 - t1336 * t102798 * t1352 - t5344 * t102801 * t1352 + t93794 - 0.10417915756705434098e0_f64 * t91078 + 2.0_f64 * t3777 * t29339 + 0.6579736267392905746e-1_f64 * t91081 + 0.3289868133696452873e-1_f64 * t97488 + 0.6579736267392905746e-1_f64 * t97491 + 0.16449340668482264365e-1_f64 * t97494 - 2.0_f64 * t1336 * t27097 * t5287 - t3777 * t29345 - 6.0_f64 * t16047 * t102801 * t19744 + 6.0_f64 * t5334 * t102801 * t5250 - t19815 * t7209;
    t102822
}
