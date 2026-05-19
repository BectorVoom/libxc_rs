//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 963/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk963<F: Float>(t29770: F, t29782: F, t29958: F, t29971: F, t29988: F, t30003: F, t30020: F, t30034: F, t2666: F, t9262: F, t9291: F, t29363: F, t29365: F, t29368: F, t29370: F, t29373: F, t29376: F, t29378: F, t29380: F, t29383: F, t29386: F, t29390: F, t29393: F, t29396: F, t29398: F, t29400: F) -> (F, F, F, F) {
    let t30037 = t29770 + t29782 + t29958 + t29971 + t29988 + t30003 + t30020 + t30034;
    let t30045 = t9262 * t2666;
    let t30048 = t2666 * t9291;
    let t30066 = -F::new(0.75e0) * t29363 - F::cast_from(0.10252083333333333334e1_f64) * t29365 - F::new(0.1875e0) * t29368 + F::new(0.161875e0) * t29370 + F::new(0.625e-1) * t29373 - F::new(0.60703125e-1) * t29376 + F::new(1.0) * t29378 + F::cast_from(0.80937499999999999999e-1_f64) * t29380 - F::new(0.1875e0) * t29383 - F::cast_from(0.80937499999999999999e-1_f64) * t29386 - F::cast_from(0.42777777777777777778e1_f64) * t29390 + F::cast_from(0.40468749999999999999e-1_f64) * t29393 + F::cast_from(0.18333333333333333333e1_f64) * t29396 - F::new(0.28125e0) * t29398 - F::cast_from(0.43166666666666666667e0_f64) * t29400;
    (t30037, t30045, t30048, t30066)
}
