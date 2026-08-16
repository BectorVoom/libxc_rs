//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1030/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1030(t2315: f64, t84: f64, t77: f64, t2251: f64, t603: f64, t2259: f64, t2311: f64, t76: f64, t10298: f64, t38: f64, t2248: f64, t2247: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25113 = t84 * t2315;
    let t25114 = t77 * t25113;
    let t25117 = t603 * t2251;
    let t25120 = t603 * t2259;
    let t25146 = t76 * t2311;
    let t25150 = t10298 * t38;
    let t25159 = t77 * t84 * t2248;
    let t25162 = t2247 * t607;
    (t25113, t25114, t25117, t25120, t25146, t25150, t25159, t25162)
}
