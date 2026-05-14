//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 893/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk893<F: Float>(t1074: F, t1295: F, t3370: F, t829: F, t1292: F, t11060: F, t11063: F, t11066: F, t1300: F, t327: F, t3373: F, t6693: F, t834: F, t1079: F, t1305: F, t1081: F, t1312: F) -> (F, F, F) {
    let t11071 = t1074 * t1295;
    let t11074 = t3370 * t829;
    let t11077 = t1074 * t1292;
    let t11082 = -0.64e0 * t11060 * t327 - 0.256e1 * t11063 * t829 - 0.384e1 * t11066 * t1295 - 0.128e1 * t3373 * t1292 - 0.384e1 * t6693 * t11071 - 0.256e1 * t1300 * t11074 - 0.128e1 * t1300 * t11077 - 0.64e0 * t834 * t11060;
    let t11087 = t1079 * t1305;
    let t11092 = t1312 * t1081;
    (t11082, t11087, t11092)
}
