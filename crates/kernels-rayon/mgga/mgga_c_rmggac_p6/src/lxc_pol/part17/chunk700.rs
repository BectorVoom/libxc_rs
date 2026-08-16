//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 700/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk700(t236: f64, t9988: f64, t7231: f64, t7230: f64, t530: f64, t8817: f64, t1743: f64, t645: f64, t903: f64, t1734: f64, t665: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9989 = t236 * t9988;
    let t9990 = t7231 * t9989;
    let t9991 = t7230 * t9990;
    let t9992 = 0.1064114997332445985e-4_f64 * t9991;
    let t9997 = t530 * t8817;
    let t9998 = 0.4726e1_f64 * t9997;
    let t9999 = t645 * t1743;
    let t10000 = t903 * t9999;
    let t10001 = 0.44903406381989282115e-1_f64 * t10000;
    let t10002 = t665 * t1734;
    let t10003 = t739 * t10002;
    (t9990, t9992, t9998, t9999, t10001, t10002, t10003)
}
