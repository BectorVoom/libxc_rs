//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1134/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1134(t121116: f64, t32213: f64, t125: f64, t4075: f64, t121035: f64, t25875: f64, t550: f64, t561: f64, t9794: f64, t2453: f64, t8571: f64, t240: f64, t27: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121117 = t121116 * t32213;
    let t121118 = 0.263521689745817692e-2_f64 * t121117;
    let t121126 = t125 * t4075;
    let t121131 = t25875 * t121035;
    let t121165 = t550 * t561;
    let t121166 = t9794 * t121165;
    let t121167 = t2453 * t8571 * t121166;
    let t121173 = t545 * t27 * t240;
    (t121118, t121126, t121131, t121165, t121166, t121167, t121173)
}
