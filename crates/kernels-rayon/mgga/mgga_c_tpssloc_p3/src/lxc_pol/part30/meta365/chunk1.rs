//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1411/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1411(t1742: f64, t3036: f64, t3503: f64, t3500: f64, t1210: f64, t11539: f64, t4724: f64, t1174: f64, t13969: f64, t4983: f64, t3515: f64, t478: f64) -> (f64, f64, f64, f64, f64) {
    let t15501 = t1742 * t3036;
    let t15502 = t3503 * t15501;
    let t15503 = t3500 * t15502;
    let t15506 = t1210 * t15501;
    let t15507 = t3500 * t15506;
    let t15522 = t11539 * t4724;
    let t15524 = t1174 * t15522 / 324.0_f64;
    let t15548 = t13969 * t4983;
    let t15550 = t3515 * t15548 / 2304.0_f64;
    let t15567 = t478 * t1742;
    (t15503, t15507, t15524, t15550, t15567)
}
