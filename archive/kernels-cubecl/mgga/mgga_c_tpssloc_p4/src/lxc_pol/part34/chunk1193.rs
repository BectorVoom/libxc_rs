//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1193/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1193<F: Float>(t1339: F, t20568: F, t6936: F, t20501: F, t6916: F, t20570: F, t6945: F, t1361: F, t20563: F, t26288: F, t20479: F, t6952: F) -> (F, F, F, F, F) {
    let t107118 = t6936 * t1339 * t20568;
    let t107120 = t6916 * t20501;
    let t107123 = t6945 * t20570;
    let t107126 = t26288 * t1361 * t20563;
    let t107133 = t6952 * t20479;
    (t107118, t107120, t107123, t107126, t107133)
}
