//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1025/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1025<F: Float>(t36085: F, t7839: F, t8908: F, t8912: F, t8970: F, t1181: F, t31567: F, t36019: F, t599: F, t1992: F, t7585: F, t7586: F, t8960: F) -> (F, F, F, F, F, F) {
    let t36086 = F::cast_from(0.10718504529517434243e-2_f64) * t36085;
    let t36087 = t7839 * t8908;
    let t36088 = F::cast_from(0.42874018118069736972e-3_f64) * t36087;
    let t36089 = t7839 * t8912;
    let t36090 = F::cast_from(0.21437009059034868486e-3_f64) * t36089;
    let t36096 = t7839 * t8970;
    let t36097 = F::cast_from(0.31448092289604152068e-3_f64) * t36096;
    let t36115 = t31567 * t1181 * t599 * t36019;
    let t36119 = t7585 * t7586 * t1992 * t8960;
    (t36086, t36088, t36090, t36097, t36115, t36119)
}
