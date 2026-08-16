//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1122/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1122(t36674: f64, t43874: f64, t43877: f64, t47594: f64, t47596: f64, t47598: f64, t47600: f64, t47602: f64, t47607: f64, t47612: f64, t47616: f64, t47621: f64, t47623: f64, t47629: f64, t47634: f64, t47639: f64, t5144: f64, t739: f64, t9530: f64) -> f64 {
    let t49277 = 0.23948483403727617128e0_f64 * t739 * t9530 * t5144 - 0.30487649791575028312e-3_f64 * t36674 - 0.14365552463988020798e-3_f64 * t47594 + 0.3405167991463827152e-4_f64 * t47596 - 0.5107751987195740728e-4_f64 * t47598 + 0.5107751987195740728e-4_f64 * t47600 + 0.1702583995731913576e-4_f64 * t47602 - 0.1702583995731913576e-4_f64 * t47607 + 0.14365552463988020798e-3_f64 * t47612 + t43874 + 2.0_f64 * t43877 + 0.18183107769496894487e-1_f64 * t47616 - 0.5107751987195740728e-4_f64 * t47621 - 0.85129199786595678799e-5_f64 * t47623 + 0.1064114997332445985e-4_f64 * t47629 + 0.1702583995731913576e-4_f64 * t47634 - 0.2553875993597870364e-4_f64 * t47639;
    t49277
}
