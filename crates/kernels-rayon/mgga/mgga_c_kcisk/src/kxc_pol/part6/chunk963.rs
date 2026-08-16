//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 963/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk963(t29770: f64, t29782: f64, t29958: f64, t29971: f64, t29988: f64, t30003: f64, t30020: f64, t30034: f64, t2666: f64, t9262: f64, t9291: f64, t29363: f64, t29365: f64, t29368: f64, t29370: f64, t29373: f64, t29376: f64, t29378: f64, t29380: f64, t29383: f64, t29386: f64, t29390: f64, t29393: f64, t29396: f64, t29398: f64, t29400: f64) -> (f64, f64, f64, f64) {
    let t30037 = t29770 + t29782 + t29958 + t29971 + t29988 + t30003 + t30020 + t30034;
    let t30045 = t9262 * t2666;
    let t30048 = t2666 * t9291;
    let t30066 = -0.75e0_f64 * t29363 - 0.10252083333333333334e1_f64 * t29365 - 0.1875e0_f64 * t29368 + 0.161875e0_f64 * t29370 + 0.625e-1_f64 * t29373 - 0.60703125e-1_f64 * t29376 + 1.0_f64 * t29378 + 0.80937499999999999999e-1_f64 * t29380 - 0.1875e0_f64 * t29383 - 0.80937499999999999999e-1_f64 * t29386 - 0.42777777777777777778e1_f64 * t29390 + 0.40468749999999999999e-1_f64 * t29393 + 0.18333333333333333333e1_f64 * t29396 - 0.28125e0_f64 * t29398 - 0.43166666666666666667e0_f64 * t29400;
    (t30037, t30045, t30048, t30066)
}
