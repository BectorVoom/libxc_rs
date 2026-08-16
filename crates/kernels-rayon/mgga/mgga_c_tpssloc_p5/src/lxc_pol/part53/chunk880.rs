//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 880/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk880(t31998: f64, t858: f64, t2053: f64, t2718: f64, t7106: f64, t865: f64, t8740: f64, t2054: f64, t24297: f64, t24305: f64, t259: f64, t2597: f64, t2713: f64, t31317: f64, t31964: f64, t31971: f64, t31974: f64, t31985: f64, t855: f64, t866: f64, t8734: f64, t8741: f64) -> (f64, f64, f64, f64) {
    let t31999 = t858 * t31998;
    let t32002 = t2718 * t2053 * t7106;
    let t32005 = t8740 * t865;
    let t32006 = t2718 * t32005;
    let t32009 = -t2597 * t8741 - 2.0_f64 * t24305 * t2054 - t31964 * t866 + 2.0_f64 * t2597 * t8734 - 2.0_f64 * t24297 * t2054 + 0.3289868133696452873e-1_f64 * t31317 - t31971 + 2.0_f64 * t2713 * t8734 + t31974 * t259 + t31985 * t259 - t855 * t31999 + 4.0_f64 * t855 * t32002 + 2.0_f64 * t855 * t32006;
    (t31999, t32002, t32006, t32009)
}
