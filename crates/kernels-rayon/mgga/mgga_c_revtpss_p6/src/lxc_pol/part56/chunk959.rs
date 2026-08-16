//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 959/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk959(t1214: f64, t494: f64, t247: f64, t3719: f64, t2148: f64, t3140: f64, t1243: f64, t479: f64, t3089: f64) -> (f64, f64, f64, f64, f64) {
    let t33406 = t494 * t1214;
    let t33408 = t247 * t3719 * t33406;
    let t33411 = t2148 * t3140;
    let t33412 = t1243 * t479;
    let t33414 = t33411 * t33412 * t3089;
    (t33406, t33408, t33411, t33412, t33414)
}
