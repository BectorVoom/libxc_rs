//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1241/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1241<F: Float>(t2926: F, t4631: F, t934: F, t2924: F, t2918: F, t4635: F, t11387: F, t1609: F, t2875: F, t11385: F, t4644: F, t945: F) -> (F, F, F, F) {
    let t15389 = t4631 * t2926;
    let t15390 = t15389 * t934;
    let t15392 = F::cast_from(0.32163958997385070134e2_f64) * t2924 * t15390;
    let t15393 = t4635 * t2918;
    let t15395 = F::cast_from(0.16081979498692535067e2_f64) * t2924 * t15393;
    let t15396 = t1609 * t11387;
    let t15397 = t15396 * t2875;
    let t15399 = F::cast_from(0.51726012919273400301e3_f64) * t11385 * t15397;
    let t15400 = t4644 * t945;
    (t15392, t15395, t15399, t15400)
}
