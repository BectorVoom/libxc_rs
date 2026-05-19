//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 777/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk777<F: Float>(t2028: F, t5437: F, t5438: F, t791: F, t10423: F, t10429: F, t10434: F, t10439: F, t10445: F, t10453: F, t10456: F, t10467: F, t10469: F, t10474: F, t10477: F, t10482: F, t10484: F, t10491: F, t10495: F, t10497: F, t5348: F, t5445: F, t5521: F) -> (F, F) {
    let t11964 = t5437 * t2028;
    let t11966 = F::new(1.0) / t5438 / t791;
    let t11967 = t11964 * t11966;
    let t11981 = F::cast_from(0.46429444444444444443e-2_f64) * t10423 - F::new(0.579e0) * t5348 * t5521 - F::cast_from(0.46429444444444444443e-2_f64) * t10429 - F::cast_from(0.34822083333333333333e-2_f64) * t10434 - F::cast_from(0.11607361111111111111e-2_f64) * t10439 + F::cast_from(0.69644166666666666666e-2_f64) * t10445 - F::new(0.223494e0) * t5445 * t11967 + F::cast_from(0.11607361111111111111e-2_f64) * t10453 + F::cast_from(0.58036805555555555555e-2_f64) * t10456 + F::cast_from(0.51588271604938271605e-2_f64) * t10467 - F::cast_from(0.46429444444444444443e-2_f64) * t10469 - F::cast_from(0.77382407407407407405e-3_f64) * t10474 - F::cast_from(0.69644166666666666666e-2_f64) * t10477 + F::cast_from(0.11607361111111111111e-2_f64) * t10482 - F::cast_from(0.46429444444444444443e-2_f64) * t10484 - F::cast_from(0.11607361111111111111e-1_f64) * t10491 + F::cast_from(0.38691203703703703703e-2_f64) * t10495 + F::cast_from(0.23214722222222222222e-2_f64) * t10497;
    (t11967, t11981)
}
