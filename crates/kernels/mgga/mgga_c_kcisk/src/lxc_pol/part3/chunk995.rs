//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 995/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk995<F: Float>(t13338: F, t13342: F, t13344: F, t13347: F, t13351: F, t13355: F, t13359: F, t13362: F, t13367: F, t13372: F, t13375: F, t13380: F) -> F {
    let t14659 = -F::cast_from(0.27857666666666666666e-1_f64) * t13338 + F::cast_from(0.69644166666666666666e-2_f64) * t13342 - F::cast_from(0.46429444444444444443e-2_f64) * t13344 - F::cast_from(0.69644166666666666666e-2_f64) * t13347 - F::cast_from(0.92858888888888888888e-2_f64) * t13351 - F::cast_from(0.15476481481481481482e-1_f64) * t13355 - F::cast_from(0.11607361111111111111e-1_f64) * t13359 - F::cast_from(0.69644166666666666665e-2_f64) * t13362 - F::cast_from(0.18571777777777777778e-1_f64) * t13367 + F::cast_from(0.11607361111111111111e-2_f64) * t13372 + F::cast_from(0.34822083333333333333e-2_f64) * t13375 - F::cast_from(0.46429444444444444443e-2_f64) * t13380;
    t14659
}
