//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 958/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk958<F: Float>(t1266: F, t321: F, t502: F, t818: F, t826: F, t1275: F, t263: F, t1277: F, t1289: F, t3358: F, t1070: F, t6651: F) -> (F, F, F, F, F, F, F, F) {
    let t11031 = t1266 * t321;
    let t11033 = t502 * t818;
    let t11034 = t11033 * t826;
    let t11035 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11034;
    let t11036 = t263 * t1275;
    let t11037 = t11036 * t1277;
    let t11039 = t3358 * t1289;
    let t11041 = t6651 * t1070;
    (t11031, t11033, t11034, t11035, t11036, t11037, t11039, t11041)
}
