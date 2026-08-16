//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2278/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2278(t23384: f64, t28470: f64, t28516: f64, t25749: f64, t7560: f64, t225: f64, t28594: f64, t1066: f64, t1635: f64, t17583: f64, t18047: f64, t18061: f64, t1920: f64, t1956: f64, t23346: f64, t25420: f64, t25757: f64, t25758: f64, t345: f64, t387: f64, t4660: f64, t5844: f64, t61621: f64, t6687: f64, t6699: f64, t6771: f64, t88882: f64, t89620: f64, t986: f64) -> f64 {
    let t99394 = t23384 * t28470;
    let t99398 = t23384 * t28516;
    let t99400 = t7560 * t25749;
    let t99415 = t28594 * t225;
    let t99422 = 4.0_f64 * t4660 * t25420 - t61621 * t1956 + 0.54831135561607547883e-2_f64 * t99394 - 0.73108180748810063845e-2_f64 * t23346 * t28516 + 0.91385225936012579807e-3_f64 * t99398 + 0.16449340668482264365e-1_f64 * t6687 * t986 * t99400 + 0.82246703342411321825e-2_f64 * t1920 * t345 * t18047 * t225 * t387 + 0.36554090374405031923e-2_f64 * t88882 - 0.82246703342411321825e-2_f64 * t6687 * t5844 * t6699 + 4.0_f64 * t6771 * t17583 - t99415 * t1066 - 2.0_f64 * t89620 * t1635 - 6.0_f64 * t25757 * t25758 * t18061;
    t99422
}
