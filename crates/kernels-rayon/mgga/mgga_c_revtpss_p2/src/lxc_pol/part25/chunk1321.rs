//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1321/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1321(t94453: f64, t94489: f64, t94532: f64, t94572: f64, t1445: f64, t2439: f64, t25916: f64, t1358: f64, t212: f64, t26034: f64, t689: f64, t10147: f64, t2027: f64, t2028: f64, t26084: f64, t4132: f64, t543: f64, t545: f64, t7279: f64, t7295: f64, t7301: f64, t94378: f64, t94388: f64, t94392: f64, t94399: f64, t94405: f64, t94409: f64, t94411: f64, t94413: f64, t9659: f64) -> (f64, f64) {
    let t94574 = t94453 + t94489 + t94532 + t94572;
    let t94580 = t2439 * t25916 * t1445;
    let t94584 = t689 * t212 * t26034 * t1358;
    let t94588 = -0.28912093960683998208e-1_f64 * t94378 - 0.39512695097613069591e1_f64 * t7279 * t9659 - 0.51405703062096148814e-2_f64 * t94388 + 0.68549505033305214441e-2_f64 * t94392 + 0.86736281882051994623e-1_f64 * t94399 - 0.65854491829355115987e0_f64 * t7279 * t10147 - 0.21684070470512998656e-1_f64 * t94405 - t94409 + 0.29272321618148349057e-1_f64 * t94411 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t94413 * t543 - 0.4336814094102599731e0_f64 * t2027 * t2028 * t545 * t94574 + 0.19514881078765566038e-2_f64 * t94580 - 0.16463622957338778996e-1_f64 * t94584 - 0.19756347548806534796e1_f64 * t26084 * t4132;
    (t94574, t94588)
}
