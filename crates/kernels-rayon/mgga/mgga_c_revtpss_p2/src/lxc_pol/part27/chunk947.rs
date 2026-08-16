//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 947/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk947(t11408: f64, t302: f64, t2944: f64, t953: f64, t2970: f64, t11132: f64, t11337: f64, t11158: f64, t11162: f64, t11167: f64, t11316: f64, t11319: f64, t11322: f64, t11326: f64, t11329: f64, t11332: f64, t11339: f64, t11343: f64, t11346: f64) -> (f64, f64, f64, f64) {
    let t11409 = t302 * t11408;
    let t11410 = t2944 * t953;
    let t11411 = t11410 * t2970;
    let t11422 = 0.16068111111111111111e1_f64 * t11132;
    let t11423 = 0.46308888888888888888e0_f64 * t11337;
    let t11428 = 0.6311625e0_f64 * t11316 - 0.104195e0_f64 * t11319 + 0.62517e0_f64 * t11322 + 0.309885e1_f64 * t11167 - 0.103295e1_f64 * t11158 - 0.41678000000000000001e0_f64 * t11326 + 0.20839e0_f64 * t11329 - 0.62517e0_f64 * t11332 - t11422 - t11423 + 0.69463333333333333335e-1_f64 * t11339 - 0.46308888888888888889e-1_f64 * t11343 - 0.104195e0_f64 * t11346 - 0.309885e1_f64 * t11162;
    (t11409, t11410, t11411, t11428)
}
