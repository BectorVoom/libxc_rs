//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2278/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2278(t24637: f64, t8009: f64, t24588: f64, t8020: f64, t1184: f64, t4929: f64, t1715: f64, t3469: f64, t24645: f64, t7999: f64, t1186: f64, t1235: f64, t15789: f64, t1716: f64, t1761: f64, t24567: f64, t24571: f64, t24589: f64, t24605: f64, t24611: f64, t24615: f64, t27406: f64, t27411: f64, t27437: f64, t27445: f64, t27453: f64, t27799: f64, t460: f64, t7283: f64, t7286: f64, t7300: f64, t86403: f64, t86475: f64) -> (f64, f64, f64, f64) {
    let t94391 = t8009 * t24637;
    let t94395 = t8020 * t24588;
    let t94400 = t4929 * t1184;
    let t94404 = t1715 * t3469;
    let t94427 = 0.14621636149762012769e-1_f64 * t7999 * t24645;
    let t94428 = -0.82246703342411321825e-2_f64 * t7283 * t1716 * t24611 - t86475 * t1761 + 0.16449340668482264365e-1_f64 * t7283 * t1186 * t94391 - 0.14621636149762012769e-1_f64 * t94395 * t24605 + 0.21932454224643019153e-1_f64 * t27406 * t24571 - 0.16449340668482264365e-1_f64 * t7283 * t94400 * t27799 - 0.82246703342411321825e-2_f64 * t7283 * t94404 * t27799 - 0.16449340668482264365e-1_f64 * t7283 * t27453 * t460 * t1235 * t7286 + 0.3289868133696452873e-1_f64 * t7283 * t24567 * t27411 + 0.3289868133696452873e-1_f64 * t7283 * t7300 * t24615 * t15789 + 0.10966227112321509577e-1_f64 * t24589 * t86403 * t27445 - 0.54831135561607547884e-2_f64 * t24589 * t86403 * t27437 - t94427;
    (t94395, t94400, t94404, t94428)
}
