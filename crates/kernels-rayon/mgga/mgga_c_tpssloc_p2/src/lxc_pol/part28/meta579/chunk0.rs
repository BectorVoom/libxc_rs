//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1863/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1863(t13193: f64, t6621: f64, t13198: f64, t23097: f64, t232: f64, t46565: f64, t815: f64, t46644: f64, t25135: f64, t838: f64, t2693: f64, t7503: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87389 = t6621 * t13193;
    let t87391 = t6621 * t13198;
    let t87395 = t23097 * t815 * t46565 * t232;
    let t87399 = t23097 * t815 * t46644 * t232;
    let t87401 = t25135 * t838;
    let t87403 = t7503 * t2693;
    (t87389, t87391, t87395, t87399, t87401, t87403)
}
