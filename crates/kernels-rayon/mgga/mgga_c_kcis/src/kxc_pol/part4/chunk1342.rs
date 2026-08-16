//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1342/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1342(t17292: f64, t4173: f64, t4160: f64, t1392: f64, t1981: f64, t1017: f64, t86: f64, t4174: f64, t4166: f64, t12231: f64, t12246: f64, t1364: f64, t16824: f64, t17007: f64, t17012: f64, t17257: f64, t17260: f64, t17263: f64, t17268: f64, t17272: f64, t17274: f64, t17277: f64, t17281: f64, t17287: f64, t17290: f64, t1944: f64, t3718: f64, t4013: f64, t5742: f64) -> (f64, f64, f64, f64) {
    let t17293 = t17292 * t4173;
    let t17294 = t4160 * t17293;
    let t17296 = t1392 * t1981;
    let t17298 = t86 * t1017 * t17296;
    let t17299 = t17298 * t4174;
    let t17301 = t17298 * t4166;
    let t17303 = -0.33163888888888888888e-2_f64 * t17007 - 0.16581944444444444444e-1_f64 * t17012 - 0.178244852896875e-2_f64 * t12231 * t16824 - 0.13345e0_f64 * t1364 * t16824 + 0.24872916666666666666e-2_f64 * t17257 + t17260 + 0.88437037037037037034e-2_f64 * t17263 + t17268 + 0.11054629629629629629e-2_f64 * t17272 - 0.44218518518518518517e-2_f64 * t17274 + t17277 - 0.16581944444444444444e-2_f64 * t17281 - 0.66725e-1_f64 * t12246 * t1944 - 0.66725e-1_f64 * t5742 * t4013 + 0.890445125e-2_f64 * t17287 * t3718 + 0.22109259259259259258e-2_f64 * t17290 - 0.5895802469135802469e-2_f64 * t17294 + 0.22109259259259259258e-2_f64 * t17299 - 0.33163888888888888888e-2_f64 * t17301;
    (t17294, t17299, t17301, t17303)
}
