//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2063/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2063(t23384: f64, t25518: f64, t10277: f64, t381: f64, t225: f64, t25608: f64, t25714: f64, t7604: f64, t82573: f64, t25718: f64, t23665: f64, t25541: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t89057 = 0.18277045187202515961e-2_f64 * t23384 * t25518;
    let t89071 = t381 * t10277;
    let t89076 = t25608 * t225;
    let t89094 = 0.54831135561607547884e-2_f64 * t23384 * t25714;
    let t89104 = t82573 * t7604;
    let t89151 = 0.18277045187202515961e-2_f64 * t23384 * t25718;
    let t89156 = 0.54831135561607547884e-2_f64 * t23665 * t25541;
    (t89057, t89071, t89076, t89094, t89104, t89151, t89156)
}
