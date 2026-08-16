//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 626/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk626(t3087: f64, t364: f64, t354: f64, t1032: f64, t1036: f64, t1004: f64, t1031: f64, t1044: f64, t248: f64, t2776: f64, t121: f64, t376: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3088 = t364 * t3087;
    let t3089 = t354 * t3088;
    let t3092 = t1032 * t1036;
    let t3094 = t1004 * t1031;
    let t3098 = t248 * t1044 * t2776;
    let t3101 = t121 * t376;
    (t3088, t3089, t3092, t3094, t3098, t3101)
}
