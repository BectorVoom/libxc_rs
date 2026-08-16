//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1099/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1099(t15199: f64, t981: f64, t11710: f64, t1483: f64, t15118: f64, t15120: f64, t15131: f64, t15135: f64, t15140: f64, t2771: f64, t373: f64, t3990: f64, t3994: f64, t4017: f64, t5018: f64, t5037: f64, t978: f64, t991: f64) -> f64 {
    let t15200 = t981 * t15199;
    let t15202 = -2.0_f64 * t11710 * t1483 + t15118 * t373 - t15120 * t991 - 6.0_f64 * t15131 * t978 + 4.0_f64 * t15135 * t978 + 2.0_f64 * t15140 * t978 - t15200 * t978 + 2.0_f64 * t2771 * t5018 - t2771 * t5037 + 4.0_f64 * t3990 * t3994 - 2.0_f64 * t3990 * t4017;
    t15202
}
