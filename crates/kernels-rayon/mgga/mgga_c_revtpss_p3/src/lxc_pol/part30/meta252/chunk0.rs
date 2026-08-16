//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1109/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1109(t1353: f64, t5651: f64, t1394: f64, t5591: f64, t1392: f64, t1395: f64, t1877: f64, t1879: f64, t539: f64, t541: f64, t5644: f64, t5650: f64) -> (f64, f64, f64) {
    let t5652 = t5651 * t1353;
    let t5655 = t1394 * t5591;
    let t5658 = 3.0_f64 * t1392 * t1879 + 3.0_f64 * t1395 * t1877 + 3.0_f64 * t539 * t5655 - t541 * t5644 - 12.0_f64 * t5650 * t5652;
    (t5652, t5655, t5658)
}
