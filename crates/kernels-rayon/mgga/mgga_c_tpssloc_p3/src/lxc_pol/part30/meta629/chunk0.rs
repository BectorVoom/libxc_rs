//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2034/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2034(t25064: f64, t81788: f64, t25135: f64, t838: f64, t2693: f64, t7503: f64, t25132: f64, t81882: f64, t6604: f64, t81968: f64, t23083: f64, t25123: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87387 = t81788 * t25064;
    let t87401 = t25135 * t838;
    let t87402 = 7.0_f64 / 1152.0_f64 * t87401;
    let t87403 = t7503 * t2693;
    let t87405 = t81882 * t25132;
    let t87407 = t81968 * t6604;
    let t87411 = t23083 * t25123;
    (t87387, t87402, t87403, t87405, t87407, t87411)
}
