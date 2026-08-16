//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1041/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1041(t1338: f64, t33822: f64, t1824: f64, t8788: f64, t115439: f64, t122503: f64, t122507: f64, t122510: f64, t122513: f64, t122518: f64, t122522: f64, t122526: f64, t122530: f64, t122533: f64, t122535: f64, t122540: f64, t124166: f64, t1332: f64, t1336: f64, t1352: f64, t32136: f64, t33841: f64, t5230: f64, t5250: f64, t5287: f64, t5334: f64, t5344: f64, t544: f64, t553: f64, t8798: f64) -> f64 {
    let t124246 = t1338 * t33822;
    let t124253 = t8788 * t1824;
    let t124273 = -t1336 * t124246 * t1352 - 0.16449340668482264365e-1_f64 * t115439 - 0.76763589786250567037e-1_f64 * t122503 - t1336 * t32136 * t5287 - t5344 * t124253 * t1352 - 0.16449340668482264365e-1_f64 * t122507 + 0.6579736267392905746e-1_f64 * t122510 - 0.3289868133696452873e-1_f64 * t122513 + 0.6579736267392905746e-1_f64 * t122518 + t544 * t553 * t124166 + 2.0_f64 * t5334 * t124253 * t5250 + t5230 * t8798 + t1332 * t33841 + 0.6579736267392905746e-1_f64 * t122522 - 0.6579736267392905746e-1_f64 * t122526 - 0.6579736267392905746e-1_f64 * t122530 + 0.3289868133696452873e-1_f64 * t122533 + 0.15352717957250113407e0_f64 * t122535 - 0.6579736267392905746e-1_f64 * t122540;
    t124273
}
