//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1071/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1071(t236: f64, t3351: f64, t6412: f64, t9188: f64, t41818: f64, t41822: f64, t47516: f64, t47520: f64, t47524: f64, t47528: f64, t47530: f64, t47532: f64, t47534: f64, t47536: f64, t47538: f64, t47541: f64, t47545: f64, t47549: f64, t47553: f64, t47557: f64, t47561: f64) -> f64 {
    let t47565 = t3351 * t9188 * t236 * t6412;
    let t47567 = -0.42564599893297839398e-5_f64 * t47516 - 0.38308139903968055457e-4_f64 * t47520 + 0.51077519871957407276e-4_f64 * t47524 + 0.12769379967989351819e-4_f64 * t47528 - 0.12769379967989351819e-4_f64 * t47530 + 0.1064114997332445985e-4_f64 * t47532 + 0.25538759935978703638e-4_f64 * t47534 - 0.25538759935978703638e-4_f64 * t47536 + 0.44903406381989282115e-1_f64 * t47538 - 0.99317399751028291929e-5_f64 * t47541 + t41818 + t41822 + 0.72042316457491791906e-3_f64 * t47545 + 0.36021158228745895953e-3_f64 * t47549 + 0.72042316457491791906e-3_f64 * t47553 - 0.76616279807936110914e-4_f64 * t47557 - 0.25538759935978703638e-4_f64 * t47561 + 0.51077519871957407276e-4_f64 * t47565;
    t47567
}
