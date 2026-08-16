//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1103/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1103(t645: f64, t79: f64, t72: f64, t605: f64, t608: f64, t625: f64, t641: f64, t71: f64, t1874: f64, t2314: f64, t4034: f64, t1266: f64, t1873: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6491 = t79 * t645;
    let t6492 = t72 * t6491;
    let t6495 = t605 * t608;
    let t6503 = 8.0_f64 / 3.0_f64 * t625;
    let t6509 = t71 * t641;
    let t6522 = 2.0_f64 * t2314 * t1874;
    let t6524 = 2.0_f64 * t4034 * t1874;
    let t6525 = t1266 * t1873;
    (t6492, t6495, t6503, t6509, t6522, t6524, t6525)
}
