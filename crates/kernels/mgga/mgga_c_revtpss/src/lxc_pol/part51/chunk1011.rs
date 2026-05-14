//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1011/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1011<F: Float>(t120138: F, t120141: F, t120149: F, t120152: F, t120156: F, t120159: F, t126368: F, t126370: F, t126376: F, t126378: F, t126380: F, t126384: F, t126386: F, t126388: F, t126390: F, t126394: F, t126397: F, t126399: F, t126403: F, t126405: F, t31794: F) -> (F,) {
    let t126408 = 0.25702851531048074406e-1 * t126368 - 0.14456046980341999104e-1 * t126370 + 0.1859366460452550541e-4 * t120138 + 0.51405703062096148812e-1 * t120141 + 0.18822977838986977999e-4 * t126376 - 0.33467254597718846885e-4 * t126378 - 0.51405703062096148813e-1 * t126380 + 0.13223814266738539448e-3 * t120149 + 0.28559868832551176308e-1 * t120152 + 0.7437465841810202164e-3 * t126384 + 0.3718732920905101082e-3 * t126386 + 0.131760844872908846e-2 * t126388 + 0.7437465841810202164e-3 * t126390 + 0.7437465841810202164e-3 * t126394 - 0.74374658418102021639e-4 * t126397 + 0.13223814266738539448e-3 * t126399 - 0.28234466758480466999e-3 * t126403 + t120156 - t120159 - 0.52041769129231196772e1 * t31794 * t126405;
    (t126408,)
}
