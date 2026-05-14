//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 404/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk404<F: Float>(t1015: F, t389: F, t1009: F, t1012: F, t422: F, t1132: F, t381: F, t13: F, t145: F, t3: F, t154: F, t265: F, t952: F, t951: F, t243: F, t483: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4112 = t1015 * t389;
    let t4114 = t1009 * t389;
    let t4116 = t1012 * t422;
    let t4118 = t1012 * t389;
    let t4120 = t381 * t1132;
    let t4124 = t1015 * t422;
    let t4129 = 1.0 / t13 / t145 * t3 / 4.0;
    let t4130 = t4129 * t154;
    let t4132 = t952 * t265;
    let t4133 = t951 * t4132;
    let t4135 = t243 * t483;
    (t4112, t4114, t4116, t4118, t4120, t4124, t4130, t4132, t4133, t4135)
}
