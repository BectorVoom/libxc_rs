//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 664/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk664(t5445: f64, t6990: f64, t6992: f64, t8868: f64, t8872: f64, t8876: f64, t8880: f64, t8884: f64, t8887: f64, t8890: f64, t8942: f64, t8949: f64, t8952: f64, t8956: f64, t8960: f64, t9163: f64) -> f64 {
    let t9257 = 0.74498e-1_f64 * t5445 * t9163 - 0.61905925925925925925e-2_f64 * t8868 - 0.23214722222222222222e-2_f64 * t8872 - 0.38691203703703703703e-3_f64 * t8876 - 0.61905925925925925925e-2_f64 * t8880 + 0.11607361111111111111e-2_f64 * t8884 - 0.34822083333333333332e-2_f64 * t8887 + 0.23214722222222222222e-2_f64 * t8890 - 0.17411041666666666666e-2_f64 * t8942 - 0.61905925925925925925e-2_f64 * t6990 + 0.23214722222222222222e-2_f64 * t6992 - 0.23214722222222222222e-2_f64 * t8949 + 0.23214722222222222222e-2_f64 * t8952 + 0.11607361111111111111e-2_f64 * t8956 + 0.19345601851851851852e-2_f64 * t8960;
    t9257
}
