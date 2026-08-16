//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1176/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1176(t15420: f64, t3447: f64, t11514: f64, t11556: f64, t11558: f64, t11561: f64, t15391: f64, t15396: f64, t15401: f64, t15405: f64, t15406: f64, t15409: f64, t15412: f64, t15415: f64) -> f64 {
    let t15422 = 0.24691358024691358024e-3_f64 * t3447 * t15420;
    let t15423 = -0.27777777777777777777e-3_f64 * t11514 + 0.37037037037037037036e-3_f64 * t11558 - 0.27777777777777777777e-3_f64 * t11561 + t11556 - 0.37037037037037037036e-3_f64 * t3447 * t15391 - 0.86419753086419753084e-3_f64 * t3447 * t15396 + t15401 - t15405 + 0.74074074074074074072e-3_f64 * t3447 * t15406 + 0.37037037037037037036e-3_f64 * t3447 * t15409 + 0.22222222222222222221e-2_f64 * t3447 * t15412 + 0.27777777777777777777e-3_f64 * t3447 * t15415 + t15422;
    t15423
}
