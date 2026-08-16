//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1030/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1030(t2185: f64, t2562: f64, t1234: f64, t921: f64, t1553: f64, t910: f64, t1569: f64, t938: f64, t113: f64, t7204: f64, t6363: f64, t920: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24064 = t2562 * t2185;
    let t24070 = t921 * t1234;
    let t24118 = t910 * t1553;
    let t24161 = t24118 * t1569;
    let t24165 = t938 * t1553;
    let t24166 = t24165 * t1569;
    let t24172 = t7204 * t113;
    let t24209 = t920 * t6363;
    (t24064, t24070, t24118, t24161, t24165, t24166, t24172, t24209)
}
