//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 626/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk626(t1143: f64, t1147: f64, t1146: f64, t445: f64, t440: f64, t1155: f64) -> (f64, f64, f64, f64) {
    let t3371 = t1143 * t1147;
    let t3374 = t1146 * t445;
    let t3375 = 1.0_f64 / t3374;
    let t3376 = t440 * t3375;
    let t3377 = t1155 * t1155;
    (t3371, t3375, t3376, t3377)
}
