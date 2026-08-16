//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 882/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk882(t342: f64, t4910: f64, t630: f64, t231: f64, t3821: f64, t13616: f64, t1526: f64, t15567: f64, t17685: f64, t17688: f64, t17695: f64, t17698: f64, t2320: f64, t343: f64, t3683: f64, t3695: f64, t3713: f64, t3827: f64, t9482: f64, t9485: f64, t9488: f64) -> f64 {
    let t17703 = t342 * t630 * t4910;
    let t17708 = t231 * t3821;
    let t17712 = t3683 + t3827 + t9482 - t9485 / 36.0_f64 - t9488 / 12.0_f64 - t17685 / 36.0_f64 - t15567 * t17688 / 9.0_f64 - t1526 * t2320 * t3695 / 12.0_f64 + t15567 * t17695 / 6.0_f64 + t1526 * t13616 * t17698 / 6.0_f64 - t17703 / 12.0_f64 - t1526 * t2320 * t3713 / 12.0_f64 - t342 * t343 * t17708 / 4.0_f64;
    t17712
}
