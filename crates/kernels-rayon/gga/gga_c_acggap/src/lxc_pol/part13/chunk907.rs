//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 907/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk907(t2067: f64, t4198: f64, t30267: f64, t3360: f64, t7643: f64, t30225: f64, t438: f64, t30248: f64, t431: f64, t30318: f64, t425: f64, t1195: f64, t7614: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30856 = t4198 * t2067;
    let t30861 = t3360 * t30267;
    let t30862 = t30861 * t7643;
    let t30866 = t30225 * t438;
    let t30868 = t30248 * t431;
    let t30872 = t30248 * t438;
    let t30874 = t30318 * t425;
    let t30876 = t7614 * t1195;
    (t30856, t30861, t30862, t30866, t30868, t30872, t30874, t30876)
}
