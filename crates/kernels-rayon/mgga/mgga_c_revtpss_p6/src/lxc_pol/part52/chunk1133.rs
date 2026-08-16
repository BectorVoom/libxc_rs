//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1133/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1133(t121099: f64, t1381: f64, t8590: f64, t27: f64, t3999: f64, t8589: f64, t25875: f64, t4021: f64, t32268: f64, t240: f64, t31752: f64, t545: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121101 = t121099 * t8590 * t1381;
    let t121102 = 0.66119071333692697238e-4_f64 * t121101;
    let t121106 = t8589 * t3999 * t27;
    let t121107 = t25875 * t121106;
    let t121108 = t121107 * t4021;
    let t121109 = 0.7437465841810202164e-4_f64 * t121108;
    let t121110 = t32268 * t121106;
    let t121111 = t121110 * t4021;
    let t121112 = 0.13223814266738539448e-3_f64 * t121111;
    let t121116 = t31752 * t545 * t843 * t240;
    (t121102, t121107, t121109, t121110, t121112, t121116)
}
