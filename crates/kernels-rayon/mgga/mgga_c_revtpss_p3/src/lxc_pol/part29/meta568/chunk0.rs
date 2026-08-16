//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1915/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1915(t14767: f64, t27159: f64, t4537: f64, t605: f64, t15071: f64, t30: f64, t25207: f64, t61203: f64, t4433: f64, t892: f64, t14749: f64, t18875: f64, t92790: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98699 = t27159 * t14767;
    let t98702 = t605 * t4537;
    let t98705 = t30 * t15071;
    let t98709 = t25207 * t61203;
    let t98713 = t892 * t605 * t4433;
    let t98716 = t27159 * t14749;
    let t98733 = t92790 * t18875;
    (t98699, t98702, t98705, t98709, t98713, t98716, t98733)
}
