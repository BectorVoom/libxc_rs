//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1952/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1952(t3698: f64, t65: f64, t5047: f64, t1234: f64, t8184: f64, t5362: f64, t7613: f64, t1230: f64, t1256: f64, t8177: f64, t2138: f64, t5261: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29054 = t65 * t3698;
    let t29055 = t29054 * t5047;
    let t29062 = t1234 * t8184;
    let t29065 = t7613 * t5362;
    let t29069 = t1230 * t8184;
    let t29072 = t8177 * t1256;
    let t29074 = t5261 * t2138;
    (t29054, t29055, t29062, t29065, t29069, t29072, t29074)
}
