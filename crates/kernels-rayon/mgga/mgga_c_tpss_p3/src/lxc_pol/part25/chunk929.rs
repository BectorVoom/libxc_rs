//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 929/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk929(t3618: f64, t8167: f64, t2158: f64, t339: f64, t790: f64, t3632: f64, t2383: f64, t3685: f64, t2169: f64, t3667: f64, t1381: f64, t8286: f64) -> (f64, f64, f64, f64, f64) {
    let t10642 = 7.0_f64 / 24.0_f64 * t8167 * t3618;
    let t10652 = t339 * t2158 * t790;
    let t10654 = 7.0_f64 / 1152.0_f64 * t10652 * t3632;
    let t10661 = 35.0_f64 / 576.0_f64 * t2383 * t3685;
    let t10678 = 7.0_f64 / 2304.0_f64 * t2169 * t3667;
    let t10679 = t8286 * t1381;
    (t10642, t10654, t10661, t10678, t10679)
}
