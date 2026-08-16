//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 390/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk390(t169: f64, t3209: f64, t299: f64, t706: f64, t286: f64, t3092: f64, t708: f64, t1687: f64, t3098: f64, t129: f64, t1692: f64, t1685: f64, t3097: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3210 = t3209 * t169;
    let t3211 = t3210 * t299;
    let t3212 = t706 * t3211;
    let t3216 = t3092 * t286 * t708;
    let t3217 = 3.0_f64 / 256.0_f64 * t3216;
    let t3218 = t3098 * t1687;
    let t3220 = t1692 * t129;
    let t3221 = t3097 * t1685;
    (t3210, t3211, t3212, t3216, t3217, t3218, t3220, t3221)
}
