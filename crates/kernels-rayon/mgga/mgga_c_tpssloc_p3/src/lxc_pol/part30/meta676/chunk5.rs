//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2112/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2112(t1862: f64, t2240: f64, t5392: f64, t1409: f64, t605: f64, t3966: f64, t72: f64, t79: f64, t2235: f64, t5399: f64, t17635: f64, t19334: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96547 = t2240 * t5392 * t1862;
    let t96551 = t605 * t1409 * t1862;
    let t96553 = t72 * t79 * t3966;
    let t96556 = t2235 * t5399;
    let t96559 = t605 * t17635;
    let t96562 = t605 * t19334;
    (t96547, t96551, t96553, t96556, t96559, t96562)
}
