//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1067/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1067(t27: f64, t3999: f64, t8589: f64, t25875: f64, t4021: f64, t32268: f64, t240: f64, t31752: f64, t545: f64, t843: f64, t32213: f64, t125: f64, t4075: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121106 = t8589 * t3999 * t27;
    let t121107 = t25875 * t121106;
    let t121108 = t121107 * t4021;
    let t121110 = t32268 * t121106;
    let t121111 = t121110 * t4021;
    let t121116 = t31752 * t545 * t843 * t240;
    let t121117 = t121116 * t32213;
    let t121126 = t125 * t4075;
    (t121107, t121108, t121110, t121111, t121116, t121117, t121126)
}
