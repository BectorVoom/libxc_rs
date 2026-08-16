//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 714/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk714(t10056: f64, t10058: f64, t1953: f64, t702: f64, t72: f64, t2435: f64, t5928: f64, t1737: f64, t699: f64, t1364: f64, t2448: f64, t623: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10377 = 0.40911992481368012596e-1_f64 * t10056;
    let t10378 = 0.5454932330849068346e-1_f64 * t10058;
    let t10379 = t1953 * t702;
    let t10380 = t72 * t10379;
    let t10381 = t5928 * t2435;
    let t10382 = 0.79828278012425390428e-1_f64 * t10381;
    let t10387 = t699 * t1737;
    let t10388 = t1364 * t10387;
    let t10389 = 0.23948483403727617128e0_f64 * t10388;
    let t10390 = t623 * t2448;
    (t10377, t10378, t10379, t10380, t10382, t10387, t10389, t10390)
}
