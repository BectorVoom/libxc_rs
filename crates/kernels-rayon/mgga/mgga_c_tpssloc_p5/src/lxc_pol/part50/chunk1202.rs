//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1202/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1202(t1920: f64, t32938: f64, t968: f64, t362: f64, t7593: f64, t1014: f64, t113578: f64, t1539: f64, t1945: f64, t23327: f64, t23478: f64, t23601: f64, t23602: f64, t23657: f64, t25429: f64, t25486: f64, t25492: f64, t25496: f64, t25510: f64, t25712: f64, t2770: f64, t2775: f64, t30877: f64, t3127: f64, t32934: f64, t3961: f64, t4347: f64, t6687: f64, t6784: f64, t6797: f64, t6799: f64, t6800: f64, t884: f64) -> f64 {
    let t119177 = t1920 * t968 * t32938;
    let t119179 = t362 * t7593;
    let t119201 = -0.16449340668482264365e-1_f64 * t6797 * t23657 * t32934 + 0.3289868133696452873e-1_f64 * t23601 * t23602 * t3127 * t1945 * t25486 + 0.16449340668482264365e-1_f64 * t6797 * t6799 * t25496 * t6800 - 0.10966227112321509577e-1_f64 * t23327 * t25510 * t1945 * t2775 * t3961 + 0.73108180748810063844e-2_f64 * t25429 * t25510 * t1945 * t2770 * t3961 + 0.54831135561607547883e-2_f64 * t119177 + 0.54831135561607547883e-2_f64 * t6687 * t6784 * t119179 * t884 - 0.16449340668482264365e-1_f64 * t23601 * t23602 * t1014 * t1945 * t25492 - 0.16449340668482264365e-1_f64 * t6687 * t25712 * t23478 * t30877 + 0.54831135561607547883e-2_f64 * t6687 * t6784 * t113578 * t1539 + 0.54831135561607547883e-2_f64 * t6687 * t6784 * t30877 * t4347;
    t119201
}
