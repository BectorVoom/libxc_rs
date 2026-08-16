//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1199/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1199(t1992: f64, t550: f64, t6976: f64, t74941: f64, t22897: f64, t3792: f64, t74949: f64, t20632: f64, t1799: f64, t6637: f64, t6888: f64, t97126: f64) -> (f64, f64, f64, f64, f64) {
    let t107281 = t1992 * t6976 * t74941 * t550;
    let t107303 = t1992 * t22897 * t74941 * t3792;
    let t107320 = t1992 * t6976 * t74949 * t550;
    let t107326 = t1992 * t6976 * t20632;
    let t107331 = t6888 * t6637 * t97126 * t1799;
    (t107281, t107303, t107320, t107326, t107331)
}
