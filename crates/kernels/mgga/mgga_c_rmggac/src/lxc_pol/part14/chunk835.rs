//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 835/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk835<F: Float>(t2004: F, t38638: F, t38594: F, t38599: F, t38604: F, t38606: F, t38608: F, t38610: F, t38615: F, t38617: F, t38619: F, t38623: F, t38624: F, t38626: F, t38628: F, t38630: F, t38632: F, t38634: F, t38636: F) -> F {
    let t38639 = t38638 * t2004;
    let t38640 = F::cast_from(0.19863479950205658386e-4_f64) * t38639;
    let t38641 = -F::cast_from(0.25538759935978703638e-4_f64) * t38594 + F::cast_from(0.25538759935978703638e-4_f64) * t38599 + F::cast_from(0.85129199786595678796e-5_f64) * t38604 - F::cast_from(0.85129199786595678796e-5_f64) * t38606 - F::cast_from(0.15243824895787514157e-3_f64) * t38608 + F::cast_from(0.15243824895787514157e-3_f64) * t38610 - F::cast_from(0.85129199786595678796e-5_f64) * t38615 - F::cast_from(0.42564599893297839398e-5_f64) * t38617 + F::cast_from(0.85129199786595678796e-5_f64) * t38619 + t38623 + F::cast_from(0.12769379967989351819e-4_f64) * t38624 - F::cast_from(0.25538759935978703638e-4_f64) * t38626 - F::cast_from(0.25538759935978703638e-4_f64) * t38628 - F::cast_from(0.12769379967989351819e-4_f64) * t38630 - F::cast_from(0.85129199786595678796e-5_f64) * t38632 - F::cast_from(0.85129199786595678796e-5_f64) * t38634 - F::cast_from(0.42564599893297839398e-5_f64) * t38636 + t38640;
    t38641
}
