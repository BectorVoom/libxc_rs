//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 962/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk962(t240: f64, t624: f64, t281: f64, t283: f64, t2909: f64, t698: f64, t3252: f64, t11145: f64, t141: f64, t11169: f64, t930: f64, t11158: f64, t11162: f64, t11167: f64, t11316: f64, t11319: f64, t11322: f64, t11326: f64, t11329: f64, t11332: f64, t11334: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11335 = t624 * t240;
    let t11337 = t281 * t11335 * t283;
    let t11338 = 0.36514074074074074075e0_f64 * t11337;
    let t11339 = t698 * t2909;
    let t11341 = t240 * t3252;
    let t11342 = t11341 * t11145;
    let t11343 = t141 * t11342;
    let t11345 = t930 * t11169;
    let t11346 = t141 * t11345;
    let t11349 = 0.3071625e0_f64 * t11316 - 0.82156666666666666668e-1_f64 * t11319 + 0.49293999999999999999e0_f64 * t11322 + 0.17938e1_f64 * t11167 - 0.59793333333333333333e0_f64 * t11158 - 0.32862666666666666666e0_f64 * t11326 + 0.16431333333333333333e0_f64 * t11329 - 0.49293999999999999999e0_f64 * t11332 - t11334 - t11338 + 0.5477111111111111111e-1_f64 * t11339 - 0.36514074074074074075e-1_f64 * t11343 - 0.82156666666666666667e-1_f64 * t11346 - 0.17938e1_f64 * t11162;
    (t11335, t11337, t11339, t11343, t11346, t11349)
}
