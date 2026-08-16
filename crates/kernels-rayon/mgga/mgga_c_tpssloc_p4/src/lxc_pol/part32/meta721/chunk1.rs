//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2291/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2291(t29624: f64, t491: f64, t1760: f64, t607: f64, t27381: f64, t8009: f64, t103132: f64, t1186: f64, t1251: f64, t17686: f64, t2128: f64, t24567: f64, t24589: f64, t24601: f64, t24602: f64, t27411: f64, t27415: f64, t27441: f64, t27445: f64, t27549: f64, t27751: f64, t27820: f64, t29803: f64, t4723: f64, t4728: f64, t4930: f64, t5398: f64, t7283: f64, t7287: f64, t8010: f64, t85642: f64, t85661: f64, t94369: f64, t94395: f64, t94458: f64, t94796: f64, t95890: f64) -> f64 {
    let t103175 = t29624 * t491;
    let t103179 = t1760 * t607;
    let t103188 = t8009 * t27381;
    let t103213 = 0.27415567780803773942e-2_f64 * t24589 * t24601 * t24602 * t5398 * t1251 - 0.14621636149762012769e-1_f64 * t94395 * t27441 - 0.10966227112321509577e-1_f64 * t24589 * t94458 * t27445 + 0.27415567780803773942e-2_f64 * t24589 * t103175 * t7287 + 0.10966227112321509577e-1_f64 * t24589 * t94369 * t4728 * t103179 - 0.73108180748810063845e-2_f64 * t27549 * t94369 * t4723 * t103179 + 0.16449340668482264365e-1_f64 * t7283 * t1186 * t103188 + 0.18277045187202515961e-2_f64 * t85661 - 0.16449340668482264365e-1_f64 * t7283 * t27751 * t27415 - 0.3289868133696452873e-1_f64 * t2128 * t27820 * t27411 + 0.16449340668482264365e-1_f64 * t7283 * t24567 * t29803 - 0.8529287754027840782e-2_f64 * t94796 * t24601 * t95890 * t17686 - 0.36554090374405031923e-2_f64 * t27549 * t24601 * t85642 * t103132 - 0.16449340668482264365e-1_f64 * t7283 * t4930 * t8010;
    t103213
}
