//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1686/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1686(t1248: f64, t1287: f64, t5230: f64, t1284: f64, t1811: f64, t1209: f64, t13392: f64, t5268: f64, t1042: f64, t1263: f64, t3362: f64, t15936: f64) -> (f64, f64, f64, f64) {
    let t17188 = t5230 * t1248 * t1287;
    let t17191 = t1284 * t1811;
    let t17192 = t1209 * t17191;
    let t17198 = t5268 * t13392;
    let t17199 = t1042 * t17198;
    let t17202 = t1263 * t3362;
    let t17203 = t17202 * t15936;
    (t17188, t17192, t17199, t17203)
}
