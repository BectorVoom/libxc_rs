//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 985/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk985(t12708: f64, t33: f64, t12649: f64, t12653: f64, t12656: f64, t12662: f64, t12665: f64, t1427: f64, t1434: f64, t2255: f64, t2304: f64, t3962: f64, t3968: f64, t3998: f64, t4018: f64, t609: f64, t642: f64, t80: f64) -> f64 {
    let t12709 = t33 * t12708;
    let t12718 = -t2255 * t1434 / 6.0_f64 - t12649 * t80 / 12.0_f64 - t12653 * t80 / 6.0_f64 - t12656 * t80 / 6.0_f64 - t3962 * t642 / 6.0_f64 - t12662 * t80 / 12.0_f64 - t12665 * t80 / 6.0_f64 - t3968 * t642 / 6.0_f64 + t12709 * t80 / 24.0_f64 + t3998 * t642 / 12.0_f64 + t1427 * t2304 / 24.0_f64 - t609 * t4018 / 6.0_f64;
    t12718
}
