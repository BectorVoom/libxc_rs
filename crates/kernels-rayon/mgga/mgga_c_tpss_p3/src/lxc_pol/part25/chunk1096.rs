//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1096/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1096(t14953: f64, t14997: f64, t15056: f64, t15115: f64, t219: f64, t5013: f64, t5017: f64, t9067: f64, t990: f64, t1482: f64, t2776: f64, t4016: f64, param_beta: f64) -> (f64, f64, f64, f64, f64) {
    let t15117 = t14953 + t14997 + t15056 + t15115;
    let t15118 = param_beta * t15117;
    let t15120 = t5013 * t219;
    let t15131 = t9067 * t5017 * t990;
    let t15135 = t2776 * t1482 * t4016;
    (t15117, t15118, t15120, t15131, t15135)
}
