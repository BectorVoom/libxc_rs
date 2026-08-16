//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 675/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk675(t4251: f64, t4292: f64, t1579: f64, t219: f64, t1148: f64, t1586: f64, t3118: f64, t1113: f64, t3126: f64, t1133: f64, t1561: f64, t4245: f64, t466: f64, param_beta: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4293 = t4251 + t4292;
    let t4294 = param_beta * t4293;
    let t4296 = t1579 * t219;
    let t4299 = t1586 * t1148;
    let t4300 = t3118 * t4299;
    let t4303 = t3126 * t1113;
    let t4307 = t1133 * t1561;
    let t4310 = t466 * t4245;
    (t4293, t4294, t4296, t4300, t4303, t4307, t4310)
}
