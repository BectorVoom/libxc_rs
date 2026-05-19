//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1189/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1189<F: Float>(t126396: F, t31827: F, t31831: F, t31755: F, t31756: F, t4364: F, t4424: F, t25317: F, t4486: F, t120138: F, t120141: F, t120149: F, t120152: F, t120156: F, t120159: F, t126368: F, t126370: F, t126376: F, t126378: F, t126380: F, t126384: F, t126386: F, t126388: F, t126390: F, t126394: F, t31794: F) -> F {
    let t126397 = t31827 * t126396;
    let t126399 = t31831 * t126396;
    let t126403 = t31755 * t4364 * t31756 * t4424;
    let t126405 = t25317 * t4486;
    let t126408 = F::cast_from(0.25702851531048074406e-1_f64) * t126368 - F::cast_from(0.14456046980341999104e-1_f64) * t126370 + F::cast_from(0.1859366460452550541e-4_f64) * t120138 + F::cast_from(0.51405703062096148812e-1_f64) * t120141 + F::cast_from(0.18822977838986977999e-4_f64) * t126376 - F::cast_from(0.33467254597718846885e-4_f64) * t126378 - F::cast_from(0.51405703062096148813e-1_f64) * t126380 + F::cast_from(0.13223814266738539448e-3_f64) * t120149 + F::cast_from(0.28559868832551176308e-1_f64) * t120152 + F::cast_from(0.7437465841810202164e-3_f64) * t126384 + F::cast_from(0.3718732920905101082e-3_f64) * t126386 + F::cast_from(0.131760844872908846e-2_f64) * t126388 + F::cast_from(0.7437465841810202164e-3_f64) * t126390 + F::cast_from(0.7437465841810202164e-3_f64) * t126394 - F::cast_from(0.74374658418102021639e-4_f64) * t126397 + F::cast_from(0.13223814266738539448e-3_f64) * t126399 - F::cast_from(0.28234466758480466999e-3_f64) * t126403 + t120156 - t120159 - F::cast_from(0.52041769129231196772e1_f64) * t31794 * t126405;
    t126408
}
