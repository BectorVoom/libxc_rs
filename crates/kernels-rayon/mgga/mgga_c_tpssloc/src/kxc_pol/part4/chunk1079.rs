//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1079/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1079(t1041: f64, t14084: f64, t14085: f64, t14117: f64, t14508: f64, t14511: f64, t1622: f64, t17734: f64, t17738: f64, t17878: f64, t17885: f64, t17890: f64, t3048: f64, t3117: f64, t3130: f64, t378: f64, t4596: f64, t4600: f64, t4636: f64, t4644: f64, t5857: f64, t5861: f64, t973: f64) -> f64 {
    let t17900 = t14084 + t14508 * t4596 / 768.0_f64 - t14511 * t4600 / 1536.0_f64 + t3130 * t17734 / 768.0_f64 + t973 * t17738 / 288.0_f64 + t17878 * t378 / 3072.0_f64 - 5.0_f64 / 2592.0_f64 * t3048 * t5861 + 5.0_f64 / 20736.0_f64 * t17885 + t3117 * t5857 / 4608.0_f64 + t1041 * t17890 / 4608.0_f64 + 5.0_f64 / 13824.0_f64 * t3117 * t5861 + t14085 * t1622 / 2304.0_f64 + t4644 * t4636 / 2304.0_f64 - t14117 / 6912.0_f64;
    t17900
}
