//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1342/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1342<F: Float>(t17292: F, t4173: F, t4160: F, t1392: F, t1981: F, t1017: F, t86: F, t4174: F, t4166: F, t12231: F, t12246: F, t1364: F, t16824: F, t17007: F, t17012: F, t17257: F, t17260: F, t17263: F, t17268: F, t17272: F, t17274: F, t17277: F, t17281: F, t17287: F, t17290: F, t1944: F, t3718: F, t4013: F, t5742: F) -> (F, F, F, F) {
    let t17293 = t17292 * t4173;
    let t17294 = t4160 * t17293;
    let t17296 = t1392 * t1981;
    let t17298 = t86 * t1017 * t17296;
    let t17299 = t17298 * t4174;
    let t17301 = t17298 * t4166;
    let t17303 = -F::cast_from(0.33163888888888888888e-2_f64) * t17007 - F::cast_from(0.16581944444444444444e-1_f64) * t17012 - F::cast_from(0.178244852896875e-2_f64) * t12231 * t16824 - F::new(0.13345e0) * t1364 * t16824 + F::cast_from(0.24872916666666666666e-2_f64) * t17257 + t17260 + F::cast_from(0.88437037037037037034e-2_f64) * t17263 + t17268 + F::cast_from(0.11054629629629629629e-2_f64) * t17272 - F::cast_from(0.44218518518518518517e-2_f64) * t17274 + t17277 - F::cast_from(0.16581944444444444444e-2_f64) * t17281 - F::new(0.66725e-1) * t12246 * t1944 - F::new(0.66725e-1) * t5742 * t4013 + F::cast_from(0.890445125e-2_f64) * t17287 * t3718 + F::cast_from(0.22109259259259259258e-2_f64) * t17290 - F::cast_from(0.5895802469135802469e-2_f64) * t17294 + F::cast_from(0.22109259259259259258e-2_f64) * t17299 - F::cast_from(0.33163888888888888888e-2_f64) * t17301;
    (t17294, t17299, t17301, t17303)
}
