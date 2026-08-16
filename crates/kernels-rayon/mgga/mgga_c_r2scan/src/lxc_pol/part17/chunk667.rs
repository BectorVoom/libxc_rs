//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 667/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk667(t4784: f64, t4877: f64, t61: f64, t41: f64, t1419: f64, t458: f64, t1416: f64, t425: f64, t1415: f64, t405: f64, t89: f64, t2098: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4878 = t4784 + t4877;
    let t4879 = t61 * t4878;
    let t4880 = t41 * t4879;
    let t4881 = t1419 * t458;
    let t4883 = t1416 * t425;
    let t4885 = t405 * t1415;
    let t4886 = t4885 * t89;
    let t4888 = 1.0_f64 / t2098;
    (t4880, t4881, t4883, t4885, t4886, t4888)
}
