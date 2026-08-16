//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2276/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2276(t23384: f64, t28510: f64, t28565: f64, t381: f64, t1065: f64, t14552: f64, t1635: f64, t17588: f64, t17635: f64, t23327: f64, t23329: f64, t23330: f64, t23346: f64, t23369: f64, t25423: f64, t25784: f64, t28470: f64, t28697: f64, t3169: f64, t4542: f64, t5398: f64, t5920: f64, t6687: f64, t6691: f64, t6816: f64, t7600: f64, t83281: f64, t88145: f64, t884: f64, t99209: f64, t99296: f64) -> f64 {
    let t99330 = t23384 * t28510;
    let t99336 = t28565 * t381;
    let t99353 = 2.0_f64 * t23369 * t5920 - 0.6092348395734171987e-3_f64 * t83281 - 2.0_f64 * t17588 * t6816 - 6.0_f64 * t3169 * t28697 - 2.0_f64 * t88145 * t1635 + 4.0_f64 * t14552 * t7600 - 0.43864908449286038307e-1_f64 * t23346 * t28470 + 0.16449340668482264365e-1_f64 * t6687 * t4542 * t25784 + 0.18277045187202515961e-2_f64 * t99330 + 0.54831135561607547883e-2_f64 * t23327 * t23329 * t99296 * t884 - 0.27415567780803773942e-2_f64 * t23327 * t99336 * t6691 - 0.27415567780803773942e-2_f64 * t23327 * t23329 * t23330 * t5398 * t1065 - 0.54831135561607547884e-2_f64 * t23327 * t23329 * t25423 * t17635 - 0.27415567780803773942e-2_f64 * t23327 * t23329 * t99209 * t884;
    t99353
}
