//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1397/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1397(t1992: f64, t550: f64, t6976: f64, t74949: f64, t20632: f64, t1799: f64, t6637: f64, t6888: f64, t97126: f64, t1825: f64, t22633: f64, t96964: f64) -> (f64, f64, f64, f64) {
    let t107320 = t1992 * t6976 * t74949 * t550;
    let t107326 = t1992 * t6976 * t20632;
    let t107331 = t6888 * t6637 * t97126 * t1799;
    let t107335 = t22633 * t6976 * t96964 * t1825;
    (t107320, t107326, t107331, t107335)
}
