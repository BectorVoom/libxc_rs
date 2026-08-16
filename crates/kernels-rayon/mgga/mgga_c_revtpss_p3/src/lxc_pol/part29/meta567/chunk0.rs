//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1914/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1914(t25207: f64, t98651: f64, t1468: f64, t2411: f64, t14365: f64, t1544: f64, t2257: f64, t4433: f64, t890: f64, t27383: f64, t61155: f64, t27375: f64, t92790: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98652 = t25207 * t98651;
    let t98658 = t2411 * t1468;
    let t98659 = t98658 * t14365;
    let t98662 = t2257 * t1544;
    let t98674 = t4433 * t890;
    let t98675 = t25207 * t98674;
    let t98688 = t27383 * t61155;
    let t98694 = t92790 * t27375;
    (t98652, t98659, t98662, t98674, t98675, t98688, t98694)
}
