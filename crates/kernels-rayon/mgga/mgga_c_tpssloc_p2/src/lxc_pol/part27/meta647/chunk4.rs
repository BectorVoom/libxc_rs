//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2234/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2234(t23384: f64, t25718: f64, t23665: f64, t25541: f64, t25545: f64, t25503: f64, t10216: f64, t381: f64, t1049: f64, t14165: f64, t14605: f64, t23327: f64, t23692: f64, t23697: f64, t25429: f64, t25470: f64, t25497: f64, t25500: f64, t25510: f64, t25536: f64, t2775: f64, t3180: f64, t3961: f64, t6680: f64, t6797: f64, t6799: f64, t6800: f64, t7610: f64, t82596: f64, t88022: f64) -> f64 {
    let t89151 = 0.18277045187202515961e-2_f64 * t23384 * t25718;
    let t89156 = 0.54831135561607547884e-2_f64 * t23665 * t25541;
    let t89158 = 0.54831135561607547884e-2_f64 * t23665 * t25545;
    let t89175 = 0.54831135561607547884e-2_f64 * t23665 * t25503;
    let t89176 = t381 * t10216;
    let t89181 = 2.0_f64 * t3180 * t25500 - 0.43864908449286038306e-1_f64 * t6680 * t25536 + 2.0_f64 * t3180 * t25497 + t89151 - 0.82246703342411321825e-2_f64 * t6797 * t82596 * t7610 + t89156 + t89158 - 0.10966227112321509577e-1_f64 * t23327 * t25510 * t1049 * t2775 * t3961 - 0.27415567780803773942e-2_f64 * t23327 * t25470 * t23692 - 0.36554090374405031923e-2_f64 * t25429 * t25470 * t23697 + 0.82246703342411321825e-2_f64 * t6797 * t6799 * t14605 * t6800 + t89175 + 0.8529287754027840782e-2_f64 * t88022 * t25510 * t89176 * t14165;
    t89181
}
