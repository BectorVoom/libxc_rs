//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 247/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk247(t1067: f64, t90: f64, t106: f64, t115: f64, t1007: f64, t1011: f64, t1026: f64, t1041: f64, t1047: f64, t1052: f64, t1059: f64, t1065: f64, t653: f64, t709: f64, t713: f64, t757: f64, t933: f64, t98: f64, t993: f64, t994: f64) -> (f64, f64, f64, f64) {
    let t1069 = t90 * t1067 / 9.0_f64;
    let t1071 = t106 * t1067 / 9.0_f64;
    let t1073 = t115 * t1067 / 9.0_f64;
    let t1075 = -t993 - t994 - t1007 * t98 / 6.0_f64 - t106 * t1011 / 6.0_f64 - t1026 * t98 / 6.0_f64 + t1041 * t98 / 6.0_f64 + t115 * t1011 / 6.0_f64 + t933 * t1047 / 36.0_f64 - t90 * t1011 / 6.0_f64 + t1052 * t713 / 6.0_f64 + t1052 * t709 / 6.0_f64 - t1059 * t98 / 6.0_f64 + t1065 + 0.10237773105191754_f64 * t653 + t1069 + t1071 - t1073 + 0.14975624337724558_f64 * t757;
    (t1069, t1071, t1073, t1075)
}
