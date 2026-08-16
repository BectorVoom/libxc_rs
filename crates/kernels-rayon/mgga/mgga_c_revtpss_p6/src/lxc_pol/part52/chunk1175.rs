//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1175/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1175(t125830: f64, t32705: f64, t32710: f64, t5710: f64, t8477: f64, t32272: f64, t33970: f64, t32275: f64, t33943: f64, t32279: f64, t125: f64, t246: f64, t32276: f64, t551: f64, t5774: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t125831 = t32705 * t125830;
    let t125833 = t32710 * t125830;
    let t125849 = t8477 * t5710;
    let t125855 = t32272 * t33970;
    let t125867 = t33943 * t32275;
    let t125868 = t125867 * t32279;
    let t125873 = t32276 * t551 * t246 * t125 * t5774;
    (t125831, t125833, t125849, t125855, t125868, t125873)
}
