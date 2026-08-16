//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 688/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk688(t678: f64, t9090: f64, t1540: f64, t687: f64, t2144: f64, t5267: f64, t1971: f64, t3351: f64, t2376: f64, t2604: f64, t333: f64, t618: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9091 = t9090 * t678;
    let t9093 = t1540 * t687;
    let t9095 = t2144 * t5267;
    let t9096 = t1971 * t9095;
    let t9097 = t3351 * t9096;
    let t9102 = t2604 * t2376;
    let t9104 = t618 * t333;
    (t9091, t9093, t9096, t9097, t9102, t9104)
}
