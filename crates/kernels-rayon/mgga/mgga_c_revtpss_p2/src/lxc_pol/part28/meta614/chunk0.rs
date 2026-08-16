//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2145/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2145(t25207: f64, t98651: f64, t1468: f64, t2411: f64, t14365: f64, t1544: f64, t2257: f64, t198: f64, t205: f64, t7086: f64, t4433: f64, t890: f64) -> (f64, f64, f64, f64, f64) {
    let t98652 = t25207 * t98651;
    let t98658 = t2411 * t1468;
    let t98659 = t98658 * t14365;
    let t98662 = t2257 * t1544;
    let t98669 = t198 * t205 * t7086;
    let t98674 = t4433 * t890;
    (t98652, t98659, t98662, t98669, t98674)
}
