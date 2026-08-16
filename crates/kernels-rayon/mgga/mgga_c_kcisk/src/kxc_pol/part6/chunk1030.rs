//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1030/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1030(t13244: f64, t1355: f64, t19182: f64, t2083: f64, t25623: f64, t306: f64, t30605: f64, t30616: f64, t30916: f64, t30938: f64, t3599: f64, t5687: f64, t7757: f64, t7764: f64) -> f64 {
    let t30941 = 3.0_f64 / 16.0_f64 * t13244 * t30616 - 3.0_f64 / 8.0_f64 * t19182 * t7757 - 3.0_f64 / 8.0_f64 * t3599 * t30916 + 3.0_f64 / 4.0_f64 * t25623 * t2083 + 3.0_f64 / 4.0_f64 * t5687 * t7764 + t1355 * t30605 / 4.0_f64 + t306 * t30938 / 2.0_f64;
    t30941
}
