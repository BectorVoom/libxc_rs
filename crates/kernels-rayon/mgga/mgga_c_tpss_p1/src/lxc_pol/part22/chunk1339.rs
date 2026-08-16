//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1339/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1339(t41371: f64, t520: f64, t1656: f64, t3326: f64, t1232: f64, t1265: f64, t3260: f64, t4460: f64, t18495: f64, t6259: f64, t3259: f64, t41590: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65738 = t41371 * t520;
    let t65783 = t1656 * t3326 * t520;
    let t65818 = t3260 * t1265 * t1232;
    let t65867 = t4460 * t1265;
    let t65871 = t6259 * t18495;
    let t65878 = t41590 * t3259;
    (t65738, t65783, t65818, t65867, t65871, t65878)
}
