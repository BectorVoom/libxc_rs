//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1151/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1151<F: Float>(t10776: F, t3308: F, t8002: F, t10772: F, t7945: F, t37883: F, t37891: F, t37893: F, t37903: F, t37905: F, t39786: F, t39789: F, t39793: F, t39795: F, t39801: F) -> F {
    let t39804 = t10776 * t3308 * t8002;
    let t39807 = t10772 * t3308 * t7945;
    let t39809 = t39786 - F::new(0.16463622957338778997e-1) * t37883 - F::new(0.2600466522016280569e1) * t39789 + t39793 - F::new(0.26198215989259945075e-1) * t39795 - F::new(0.85366933852867742945e0) * t37891 + F::new(0.12805040077930161442e0) * t37893 - F::new(0.31147743054556651236e-1) * t37903 - F::new(0.23804984598836975486e-2) * t37905 + F::new(0.21831846657716620896e-2) * t39801 + F::new(0.86682217400542685632e-1) * t39804 + F::new(0.13002332610081402845e0) * t39807;
    t39809
}
