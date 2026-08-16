//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3481/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3481(t11994: f64, t19920: f64, t4866: f64, t373: f64, t19692: f64, t3127: f64, t3172: f64, t19650: f64, t4837: f64, t1042: f64, t15697: f64, t15728: f64, t1671: f64, t19651: f64, t3150: f64, t3155: f64, t53298: f64, t53300: f64, t53302: f64, t53308: f64, t53317: f64, t53326: f64, t55141: f64, t55195: f64) -> (f64, f64, f64) {
    let t65471 = t11994 * t19920;
    let t65481 = t4866 * t4866;
    let t65482 = t373 * t65481;
    let t65488 = t3127 * t3172 * t19692;
    let t65493 = t4837 * t3172 * t19650;
    let t65497 = -0.3811023832717309953e-3_f64 * t65471 + 0.96545937095505185476e-2_f64 * t53298 + 0.10162730220579493208e-2_f64 * t53300 - 0.30488190661738479624e-2_f64 * t53302 + 0.28582678745379824648e-3_f64 * t53308 - 0.19055119163586549765e-3_f64 * t53317 + 0.1270341277572436651e-3_f64 * t53326 - 0.57165357490759649296e-3_f64 * t55141 * t15697 + 0.85748036236139473944e-3_f64 * t3150 * t1042 * t65482 * t3155 - 0.31758531939310916276e-3_f64 * t65488 - 0.30488190661738479624e-2_f64 * t15728 * t19651 + 0.3811023832717309953e-3_f64 * t65493 + 0.42874018118069736972e-3_f64 * t55195 * t1671;
    (t65481, t65482, t65497)
}
