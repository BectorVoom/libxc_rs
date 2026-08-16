//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1199/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1199(t23384: f64, t32993: f64, t113149: f64, t113207: f64, t14545: f64, t1539: f64, t1599: f64, t1955: f64, t23327: f64, t23329: f64, t23394: f64, t25420: f64, t25429: f64, t25452: f64, t25731: f64, t25755: f64, t25757: f64, t2770: f64, t3026: f64, t30782: f64, t30800: f64, t30900: f64, t3169: f64, t32965: f64, t32969: f64, t3961: f64, t43603: f64, t4542: f64, t4660: f64, t4664: f64, t6687: f64, t6704: f64, t6705: f64, t6771: f64, t6776: f64, t82502: f64, t8376: f64, t8380: f64, t8396: f64, t8397: f64, t88112: f64, t88162: f64, t89598: f64) -> f64 {
    let t119033 = t23384 * t32993;
    let t119065 = -t3169 * t32965 + 24.0_f64 * t25757 * t43603 * t8396 * t4664 + 4.0_f64 * t6771 * t25420 - t3026 * t32965 - 0.73108180748810063844e-2_f64 * t25429 * t88112 * t1955 * t2770 * t3961 + 0.54831135561607547883e-2_f64 * t23327 * t82502 * t32969 - 0.54831135561607547883e-2_f64 * t119033 - 0.16449340668482264365e-1_f64 * t6687 * t4542 * t8376 - 0.16449340668482264365e-1_f64 * t6687 * t1599 * t30800 + 0.18277045187202515961e-2_f64 * t113207 - 0.16449340668482264365e-1_f64 * t6687 * t6704 * t6705 * t25731 - 0.16449340668482264365e-1_f64 * t6687 * t89598 * t8380 + 0.54831135561607547883e-2_f64 * t23327 * t88162 * t30782 + 0.3289868133696452873e-1_f64 * t6687 * t6704 * t23394 * t25452 - 0.54831135561607547883e-2_f64 * t23327 * t23329 * t113149 * t1539 + 4.0_f64 * t25755 * t6776 + 2.0_f64 * t14545 * t8397 - t4660 * t30900;
    t119065
}
