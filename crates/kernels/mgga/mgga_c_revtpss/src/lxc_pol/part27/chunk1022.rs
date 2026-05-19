//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1022/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1022<F: Float>(t12334: F, t12356: F, t1150: F, t1131: F, t1126: F, t3383: F, t3386: F, t12228: F, t3433: F, t12295: F, t12292: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F) -> (F, F, F, F) {
    let t12357 = t12334 + t12356;
    let t12358 = t12357 * t1150;
    let t12360 = F::new(1.0) * t1131 * t12358;
    let t12361 = t1126 * t3383;
    let t12363 = F::new(6.0) * t12361 * t3386;
    let t12364 = t12228 * t1150;
    let t12366 = F::new(6.0) * t3433 * t12364;
    let t12367 = F::cast_from(0.28842592592592592592e-1_f64) * t12295;
    let t12378 = -t12367 + F::cast_from(0.12361111111111111111e-1_f64) * t12297 + F::cast_from(0.61805555555555555556e-2_f64) * t12299 - F::cast_from(0.18541666666666666667e-1_f64) * t12301 - F::cast_from(0.92708333333333333334e-2_f64) * t12303 + F::cast_from(0.10300925925925925926e-1_f64) * t12307 - F::cast_from(0.37083333333333333333e-1_f64) * t12310 - F::cast_from(0.18541666666666666666e-1_f64) * t12292 + F::cast_from(0.55625000000000000001e-1_f64) * t12314 + F::cast_from(0.55625000000000000001e-1_f64) * t12317 + F::cast_from(0.92708333333333333333e-2_f64) * t12320;
    (t12360, t12363, t12366, t12378)
}
