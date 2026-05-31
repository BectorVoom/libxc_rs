//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 664/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk664<F: Float>(t5445: F, t6990: F, t6992: F, t8868: F, t8872: F, t8876: F, t8880: F, t8884: F, t8887: F, t8890: F, t8942: F, t8949: F, t8952: F, t8956: F, t8960: F, t9163: F) -> F {
    let t9257 = F::cast_from(0.74498e-1_f64) * t5445 * t9163 - F::cast_from(0.61905925925925925925e-2_f64) * t8868 - F::cast_from(0.23214722222222222222e-2_f64) * t8872 - F::cast_from(0.38691203703703703703e-3_f64) * t8876 - F::cast_from(0.61905925925925925925e-2_f64) * t8880 + F::cast_from(0.11607361111111111111e-2_f64) * t8884 - F::cast_from(0.34822083333333333332e-2_f64) * t8887 + F::cast_from(0.23214722222222222222e-2_f64) * t8890 - F::cast_from(0.17411041666666666666e-2_f64) * t8942 - F::cast_from(0.61905925925925925925e-2_f64) * t6990 + F::cast_from(0.23214722222222222222e-2_f64) * t6992 - F::cast_from(0.23214722222222222222e-2_f64) * t8949 + F::cast_from(0.23214722222222222222e-2_f64) * t8952 + F::cast_from(0.11607361111111111111e-2_f64) * t8956 + F::cast_from(0.19345601851851851852e-2_f64) * t8960;
    t9257
}
