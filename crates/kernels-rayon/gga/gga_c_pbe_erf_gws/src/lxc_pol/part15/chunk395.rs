//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 395/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk395(t1336: f64, t88: f64, t147: f64, t784: f64, t169: f64, t242: f64, t299: f64, t535: f64, t700: f64, t766: f64, t145: f64, t34: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1337 = t1336 * t88;
    let t1338 = 12.0_f64 * t1337;
    let t1339 = t784 * t147;
    let t1342 = 0.14149184788746388121e0_f64 * t169 * t1339 * t242;
    let t1343 = t299 * t535;
    let t1345 = t169 * t1343 * t242;
    let t1349 = 0.1061188859155979109e0_f64 * t169 * t766 * t700;
    let t1350 = 2.0_f64 * t145;
    let t1351 = t34 * t532;
    (t1338, t1339, t1342, t1343, t1345, t1349, t1350, t1351)
}
