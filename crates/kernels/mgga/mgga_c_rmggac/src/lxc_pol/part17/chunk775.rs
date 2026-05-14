//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 775/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk775<F: Float>(t1971: F, t333: F, t511: F, t6108: F, t7365: F, t352: F, t515: F, t1707: F, t3351: F, t498: F, t7248: F, t34659: F, t34662: F, t34665: F, t38312: F, t38315: F, t38318: F, t38322: F, t38326: F, t44580: F, t44584: F, t44590: F, t44595: F, t44600: F, t44605: F) -> (F,) {
    let t44610 = t7365 * t1971 * t511 * t6108 * t333;
    let t44615 = t7365 * t1971 * t515 * t6108 * t352;
    let t44620 = t3351 * t7248 * t511 * t1707 * t498;
    let t44622 = 0.16260079888840015101e-2 * t38312 + t38315 - 0.66671395154821946449e-1 * t38318 + 0.33335697577410973224e-1 * t34659 - 0.3903207359137154578e-3 * t38322 + 0.60975299583150056628e-3 * t38326 + 0.14905073231436680509e-2 * t34662 + 0.14905073231436680509e-2 * t34665 - 0.42564599893297839398e-5 * t44580 + 0.17025839957319135759e-4 * t44584 + 0.85129199786595678796e-5 * t44590 + 0.53205749866622299248e-5 * t44595 - 0.85129199786595678796e-5 * t44600 + 0.25538759935978703639e-4 * t44605 - 0.25538759935978703639e-4 * t44610 - 0.85129199786595678796e-5 * t44615 + 0.76616279807936110914e-4 * t44620;
    (t44622,)
}
