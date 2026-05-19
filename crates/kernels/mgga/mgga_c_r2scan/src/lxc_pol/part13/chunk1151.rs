//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1151/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1151<F: Float>(t10776: F, t3308: F, t8002: F, t10772: F, t7945: F, t37883: F, t37891: F, t37893: F, t37903: F, t37905: F, t39786: F, t39789: F, t39793: F, t39795: F, t39801: F) -> F {
    let t39804 = t10776 * t3308 * t8002;
    let t39807 = t10772 * t3308 * t7945;
    let t39809 = t39786 - F::cast_from(0.16463622957338778997e-1_f64) * t37883 - F::cast_from(0.2600466522016280569e1_f64) * t39789 + t39793 - F::cast_from(0.26198215989259945075e-1_f64) * t39795 - F::cast_from(0.85366933852867742945e0_f64) * t37891 + F::cast_from(0.12805040077930161442e0_f64) * t37893 - F::cast_from(0.31147743054556651236e-1_f64) * t37903 - F::cast_from(0.23804984598836975486e-2_f64) * t37905 + F::cast_from(0.21831846657716620896e-2_f64) * t39801 + F::cast_from(0.86682217400542685632e-1_f64) * t39804 + F::cast_from(0.13002332610081402845e0_f64) * t39807;
    t39809
}
