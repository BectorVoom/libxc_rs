//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 865/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk865(t12889: f64, t12890: f64, t16668: f64, t16673: f64, t16677: f64, t16679: f64, t16684: f64, t16689: f64, t16692: f64, t16696: f64, t16699: f64, t12343: f64, t12346: f64, t12359: f64, t12362: f64, t12571: f64, t12891: f64, t12897: f64, t12911: f64, t12914: f64, t16706: f64, t9383: f64) -> (f64, f64) {
    let t17454 = -4.0_f64 / 9.0_f64 * t16668 - 4.0_f64 / 9.0_f64 * t16673 + 4.0_f64 / 27.0_f64 * t16677 - 2.0_f64 / 27.0_f64 * t16679 + t16684 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t16689 + 8.0_f64 / 9.0_f64 * t16692 + t16696 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t16699 - t12889 - t12890;
    let t17459 = t12891 - t12897 - t12343 - t12346 - t12911 + 4.0_f64 / 27.0_f64 * t12359 - 8.0_f64 / 81.0_f64 * t12362 - t9383 + t12914 - 8.0_f64 / 27.0_f64 * t12571 - 2.0_f64 / 27.0_f64 * t16706;
    (t17454, t17459)
}
