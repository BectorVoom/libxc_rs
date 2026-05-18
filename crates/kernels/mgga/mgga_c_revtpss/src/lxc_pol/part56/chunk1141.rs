//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1141/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1141<F: Float>(t126396: F, t31827: F, t31831: F, t31755: F, t31756: F, t4364: F, t4424: F, t25317: F, t4486: F, t120138: F, t120141: F, t120149: F, t120152: F, t120156: F, t120159: F, t126368: F, t126370: F, t126376: F, t126378: F, t126380: F, t126384: F, t126386: F, t126388: F, t126390: F, t126394: F, t31794: F) -> F {
    let t126397 = t31827 * t126396;
    let t126399 = t31831 * t126396;
    let t126403 = t31755 * t4364 * t31756 * t4424;
    let t126405 = t25317 * t4486;
    let t126408 = F::new(0.25702851531048074406e-1) * t126368 - F::new(0.14456046980341999104e-1) * t126370 + F::new(0.1859366460452550541e-4) * t120138 + F::new(0.51405703062096148812e-1) * t120141 + F::new(0.18822977838986977999e-4) * t126376 - F::new(0.33467254597718846885e-4) * t126378 - F::new(0.51405703062096148813e-1) * t126380 + F::new(0.13223814266738539448e-3) * t120149 + F::new(0.28559868832551176308e-1) * t120152 + F::new(0.7437465841810202164e-3) * t126384 + F::new(0.3718732920905101082e-3) * t126386 + F::new(0.131760844872908846e-2) * t126388 + F::new(0.7437465841810202164e-3) * t126390 + F::new(0.7437465841810202164e-3) * t126394 - F::new(0.74374658418102021639e-4) * t126397 + F::new(0.13223814266738539448e-3) * t126399 - F::new(0.28234466758480466999e-3) * t126403 + t120156 - t120159 - F::new(0.52041769129231196772e1) * t31794 * t126405;
    t126408
}
