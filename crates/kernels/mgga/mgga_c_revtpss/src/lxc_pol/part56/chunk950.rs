//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 950/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk950<F: Float>(t4241: F, t8441: F, t8621: F, t1493: F, t640: F, t4237: F, t84: F, t1470: F, t644: F, t8442: F, t119457: F, t36: F, t606: F, t60221: F, t8435: F, t33612: F) -> (F, F, F, F, F, F, F, F) {
    let t125228 = t8621 * t8441 * t4241;
    let t125244 = t8621 * t640 * t1493;
    let t125248 = t8621 * t84 * t4237;
    let t125260 = t1470 * t644;
    let t125261 = t8442 * t125260;
    let t125268 = t1470 * t640;
    let t125269 = t119457 * t125268;
    let t125279 = t1493 * t36 * t606;
    let t125280 = t119457 * t125279;
    let t125283 = t60221 * t8435;
    let t125290 = t8621 * t33612 * t644;
    (t125228, t125244, t125248, t125261, t125269, t125280, t125283, t125290)
}
