//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1211/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1211(t32970: f64, t82431: f64, t23384: f64, t32981: f64, t32998: f64, t113243: f64, t1927: f64, t1946: f64, t1956: f64, t23327: f64, t23365: f64, t23372: f64, t254: f64, t25406: f64, t25424: f64, t25429: f64, t25431: f64, t25732: f64, t25759: f64, t25778: f64, t25815: f64, t3026: f64, t30805: f64, t30861: f64, t30868: f64, t30904: f64, t32909: f64, t32980: f64, t4548: f64, t4557: f64, t4660: f64, t6687: f64, t6771: f64, t6776: f64, t7600: f64, t89666: f64) -> f64 {
    let t119444 = t82431 * t32970;
    let t119446 = t23384 * t32981;
    let t119467 = t23384 * t32998;
    let t119485 = 0.16449340668482264365e-1_f64 * t1927 * t4548 * t30861 - 0.18277045187202515961e-2_f64 * t119444 + 0.10966227112321509577e-1_f64 * t119446 - 2.0_f64 * t89666 * t1956 + 0.3289868133696452873e-1_f64 * t6687 * t23365 * t32980 - 6.0_f64 * t4557 * t30805 + 4.0_f64 * t23372 * t7600 - 6.0_f64 * t3026 * t32909 - 12.0_f64 * t1946 * t254 * t25759 + 0.3289868133696452873e-1_f64 * t6687 * t25406 * t30904 - 6.0_f64 * t4660 * t30805 - 0.54831135561607547883e-2_f64 * t119467 - 0.16449340668482264365e-1_f64 * t6687 * t25406 * t30868 - 0.54831135561607547883e-2_f64 * t23327 * t113243 * t25815 - 0.10966227112321509577e-1_f64 * t23327 * t113243 * t25424 + 0.73108180748810063844e-2_f64 * t25429 * t113243 * t25431 - 2.0_f64 * t6771 * t25732 + 4.0_f64 * t25778 * t6776;
    t119485
}
