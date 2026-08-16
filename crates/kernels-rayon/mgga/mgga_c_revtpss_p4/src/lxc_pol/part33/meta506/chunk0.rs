//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1825/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1825(t27137: f64, t651: f64, t7235: f64, t7935: f64, t1353: f64, t1907: f64, t8717: f64, t25082: f64, t1962: f64, t198: f64, t205: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27139 = 2.0_f64 * t651 * t27137;
    let t27152 = t7235 * t7935;
    let t27153 = t1907 * t1353;
    let t27154 = t8717 * t27153;
    let t27156 = 3.0_f64 * t25082 * t27154;
    let t27158 = t198 * t205 * t1962;
    (t27139, t27152, t27153, t27154, t27156, t27158)
}
