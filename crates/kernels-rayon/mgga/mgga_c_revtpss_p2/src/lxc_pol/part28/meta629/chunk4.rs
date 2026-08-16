//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2269/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2269(t1916: f64, t26127: f64, t26130: f64, t1459: f64, t28265: f64, t26124: f64, t28264: f64, t4292: f64, t572: f64, t13514: f64, t7330: f64, t1518: f64, t1936: f64, t2371: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t101570 = 6.0_f64 * t1916 * t26127;
    let t101572 = 3.0_f64 * t1916 * t26130;
    let t101576 = 12.0_f64 * t1459 * t28265;
    let t101578 = 12.0_f64 * t1916 * t26124;
    let t101583 = 12.0_f64 * t572 * t28264 * t4292;
    let t101586 = 6.0_f64 * t572 * t7330 * t13514;
    let t101590 = 6.0_f64 * t572 * t2371 * t1936 * t1518;
    (t101570, t101572, t101576, t101578, t101583, t101586, t101590)
}
