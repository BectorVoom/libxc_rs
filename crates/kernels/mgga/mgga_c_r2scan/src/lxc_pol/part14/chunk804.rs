//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 804/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk804<F: Float>(t1422: F, t899: F, t1416: F, t1419: F, t7055: F, t7058: F, t7091: F, t7093: F, t7095: F, t7097: F, t7098: F, t7101: F, t7104: F, t881: F) -> (F, F, F, F) {
    let t7107 = t1422 * t899;
    let t7108 = F::cast_from(32.0_f64) * t7107;
    let t7109 = t1416 * t899;
    let t7110 = F::cast_from(20.0_f64) * t7109;
    let t7111 = t1419 * t899;
    let t7112 = F::cast_from(12.0_f64) * t7111;
    let t7113 = -t7055 - t7058 - t7091 - t7093 - t7095 + t7097 - F::cast_from(0.2363e1_f64) * t881 * t7098 - F::cast_from(0.4726e1_f64) * t881 * t7101 - F::cast_from(0.2363e1_f64) * t881 * t7104 + t7108 - t7110 - t7112;
    (t7108, t7110, t7112, t7113)
}
