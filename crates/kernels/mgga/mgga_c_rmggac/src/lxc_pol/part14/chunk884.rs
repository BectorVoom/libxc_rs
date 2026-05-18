//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 884/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk884<F: Float>(t638: F, t7292: F, t8475: F, t289: F, t39290: F, t39293: F, t39296: F, t39297: F, t39301: F, t39306: F, t39308: F, t39310: F, t39312: F, t39314: F, t39316: F, t39319: F, t39320: F, t39323: F, t39325: F, t39330: F) -> F {
    let t39333 = t638 * t7292 * t8475;
    let t39335 = -t39290 - F::new(0.25538759935978703639e-4) * t39293 + t39296 - F::new(0.42564599893297839398e-5) * t39297 + F::new(0.11971293719990017331e-4) * t39301 + F::new(0.53205749866622299248e-5) * t39306 - F::new(0.33105799917009430643e-4) * t39308 - F::new(0.42564599893297839398e-5) * t39310 + F::new(0.1064114997332445985e-4) * t39312 - F::new(0.31923449919973379548e-4) * t39314 + F::new(0.31923449919973379548e-4) * t39316 + t39319 - F::new(0.4726e1) * t289 * t39320 + F::new(0.85129199786595678796e-5) * t39323 - F::new(0.85129199786595678796e-5) * t39325 + F::new(0.1064114997332445985e-4) * t39330 + F::new(0.81300399444200075504e-3) * t39333;
    t39335
}
