//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1128/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1128(t12772: f64, t5406: f64, t3625: f64, t1802: f64, t474: f64, t3089: f64, t3717: f64, t1284: f64, t5219: f64, t3624: f64, t1230: f64, t5390: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17384 = t12772 * t5406;
    let t17386 = 0.19055119163586549765e-3_f64 * t3625 * t17384;
    let t17394 = t474 * t1802;
    let t17395 = t17394 * t3089;
    let t17396 = t3717 * t17395;
    let t17400 = t5219 * t1284;
    let t17401 = t17400 * t3624;
    let t17412 = t1230 * t5390;
    (t17386, t17394, t17395, t17396, t17401, t17412)
}
