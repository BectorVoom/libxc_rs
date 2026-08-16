//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1156/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1156(t1561: f64, t4293: f64, t1578: f64, t4245: f64, t1133: f64, t5242: f64, t15796: f64, t466: f64, t3139: f64, t1113: f64, t450: f64, t9080: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15975 = t4293 * t1561;
    let t15979 = t1578 * t4245;
    let t15989 = t1133 * t5242;
    let t15992 = t466 * t15796;
    let t15999 = t3139 * t4245;
    let t16004 = t9080 * t1113 * t450;
    (t15975, t15979, t15989, t15992, t15999, t16004)
}
