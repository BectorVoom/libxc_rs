//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2086/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2086(t25365: f64, t868: f64, t25373: f64, t58009: f64, t4255: f64, t22960: f64, t1408: f64, t1877: f64, t1915: f64, t2249: f64, t22959: f64, t22964: f64, t23286: f64, t23299: f64, t25013: f64, t25028: f64, t2522: f64, t25358: f64, t25372: f64, t47645: f64, t6666: f64, t7475: f64, t7476: f64, t7541: f64, t7545: f64, t81525: f64, t86757: f64, t86764: f64, t86771: f64, t86775: f64) -> (f64, f64, f64) {
    let t86781 = t25365 * t868;
    let t86782 = t25373 * t86781;
    let t86794 = t25373 * t58009;
    let t86797 = t4255 * t868;
    let t86798 = t22960 * t86797;
    let t86801 = t86757 - t1877 * t81525 * t7545 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t23286 * t7475 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t86764 + 3.0_f64 * t47645 * t7476 + 2.0_f64 * t25372 * t86771 + t86775 + t1877 * t23286 * t1408 / 2.0_f64 - t1877 * t25358 * t23299 + 6.0_f64 * t22959 * t86782 + 3.0_f64 * t2522 * t7541 * t22964 + t1877 * t7541 * t2249 / 2.0_f64 + 3.0_f64 * t2522 * t6666 * t25028 + 2.0_f64 * t25372 * t86794 - 6.0_f64 * t25013 * t86798;
    (t86781, t86797, t86801)
}
