//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1071/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1071<F: Float>(t236: F, t3351: F, t6412: F, t9188: F, t41818: F, t41822: F, t47516: F, t47520: F, t47524: F, t47528: F, t47530: F, t47532: F, t47534: F, t47536: F, t47538: F, t47541: F, t47545: F, t47549: F, t47553: F, t47557: F, t47561: F) -> F {
    let t47565 = t3351 * t9188 * t236 * t6412;
    let t47567 = -F::cast_from(0.42564599893297839398e-5_f64) * t47516 - F::cast_from(0.38308139903968055457e-4_f64) * t47520 + F::cast_from(0.51077519871957407276e-4_f64) * t47524 + F::cast_from(0.12769379967989351819e-4_f64) * t47528 - F::cast_from(0.12769379967989351819e-4_f64) * t47530 + F::cast_from(0.1064114997332445985e-4_f64) * t47532 + F::cast_from(0.25538759935978703638e-4_f64) * t47534 - F::cast_from(0.25538759935978703638e-4_f64) * t47536 + F::cast_from(0.44903406381989282115e-1_f64) * t47538 - F::cast_from(0.99317399751028291929e-5_f64) * t47541 + t41818 + t41822 + F::cast_from(0.72042316457491791906e-3_f64) * t47545 + F::cast_from(0.36021158228745895953e-3_f64) * t47549 + F::cast_from(0.72042316457491791906e-3_f64) * t47553 - F::cast_from(0.76616279807936110914e-4_f64) * t47557 - F::cast_from(0.25538759935978703638e-4_f64) * t47561 + F::cast_from(0.51077519871957407276e-4_f64) * t47565;
    t47567
}
