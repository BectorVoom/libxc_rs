//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 913/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk913<F: Float>(t8577: F, t9153: F, t34922: F, t34927: F, t34931: F, t39119: F, t42856: F, t45226: F, t45234: F, t45240: F, t45242: F, t45244: F, t45249: F, t45254: F, t45259: F, t45264: F, t45266: F, t45272: F) -> F {
    let t45274 = t8577 * t9153;
    let t45276 = F::cast_from(0.17961362552795712846e0_f64) * t45226 - t34922 + F::cast_from(0.34200192530023447503e-6_f64) * t34927 + F::cast_from(0.34200192530023447503e-6_f64) * t34931 + F::cast_from(0.20496175532535769484e-3_f64) * t39119 + F::cast_from(0.42564599893297839398e-5_f64) * t45234 + t42856 - F::cast_from(0.85129199786595678796e-5_f64) * t45240 - F::cast_from(0.19863479950205658386e-4_f64) * t45242 - F::cast_from(0.59590439850616975155e-4_f64) * t45244 - F::cast_from(0.1064114997332445985e-4_f64) * t45249 + F::cast_from(0.3192344991997337955e-4_f64) * t45254 - F::cast_from(0.3192344991997337955e-4_f64) * t45259 - F::cast_from(0.212822999466489197e-4_f64) * t45264 + F::cast_from(0.85129199786595678796e-5_f64) * t45266 + F::cast_from(0.42564599893297839398e-5_f64) * t45272 - F::cast_from(0.25538759935978703638e-4_f64) * t45274;
    t45276
}
