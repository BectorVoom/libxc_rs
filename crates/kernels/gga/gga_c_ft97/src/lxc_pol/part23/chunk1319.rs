//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1319/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1319<F: Float>(t28533: F, t4255: F, t10447: F, t10703: F, t112725: F, t112765: F, t112773: F, t112775: F, t112777: F, t112778: F, t112784: F, t11593: F, t15195: F, t15294: F, t15299: F, t15312: F, t18497: F, t1901: F, t19460: F, t19527: F, t24886: F, t29150: F, t29202: F, t29207: F, t31713: F, t4260: F, t56180: F, t7101: F) -> (F, F) {
    let t125971 = t28533 * t4255;
    let t125984 = t1901 * t24886 * t19527 / 9.0 + 2.0 / 9.0 * t1901 * t56180 * t7101 + 2.0 / 9.0 * t1901 * t15195 * t29150 - 4.0 / 9.0 * t1901 * t15312 * t112725 * t4260 + 2.0 / 9.0 * t1901 * t10447 * t31713 + 2.0 / 9.0 * t1901 * t10703 * t29202 * t19460 - 4.0 / 9.0 * t1901 * t15299 * t125971 - 2.0 / 9.0 * t1901 * t10703 * t28533 * t4260 - 8.0 / 27.0 * t11593 * t15294 * t29207 * t18497 + t112765 + t112773 + t112775 + t112777 - 8.0 / 27.0 * t112778 + t112784;
    (t125971, t125984)
}
