//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2278/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2278<F: Float>(t23384: F, t28470: F, t28516: F, t25749: F, t7560: F, t225: F, t28594: F, t1066: F, t1635: F, t17583: F, t18047: F, t18061: F, t1920: F, t1956: F, t23346: F, t25420: F, t25757: F, t25758: F, t345: F, t387: F, t4660: F, t5844: F, t61621: F, t6687: F, t6699: F, t6771: F, t88882: F, t89620: F, t986: F) -> F {
    let t99394 = t23384 * t28470;
    let t99398 = t23384 * t28516;
    let t99400 = t7560 * t25749;
    let t99415 = t28594 * t225;
    let t99422 = F::cast_from(4.0_f64) * t4660 * t25420 - t61621 * t1956 + F::cast_from(0.54831135561607547883e-2_f64) * t99394 - F::cast_from(0.73108180748810063845e-2_f64) * t23346 * t28516 + F::cast_from(0.91385225936012579807e-3_f64) * t99398 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t986 * t99400 + F::cast_from(0.82246703342411321825e-2_f64) * t1920 * t345 * t18047 * t225 * t387 + F::cast_from(0.36554090374405031923e-2_f64) * t88882 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t5844 * t6699 + F::cast_from(4.0_f64) * t6771 * t17583 - t99415 * t1066 - F::cast_from(2.0_f64) * t89620 * t1635 - F::cast_from(6.0_f64) * t25757 * t25758 * t18061;
    t99422
}
