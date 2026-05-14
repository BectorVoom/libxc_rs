//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1030/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1030<F: Float>(t37883: F, t37891: F, t37893: F, t37903: F, t37905: F, t39786: F, t39789: F, t39793: F, t39795: F, t39801: F, t39804: F, t39807: F, t10781: F, t7535: F, t10757: F, t980: F) -> (F, F, F) {
    let t39809 = t39786 - 0.16463622957338778997e-1 * t37883 - 0.2600466522016280569e1 * t39789 + t39793 - 0.26198215989259945075e-1 * t39795 - 0.85366933852867742945e0 * t37891 + 0.12805040077930161442e0 * t37893 - 0.31147743054556651236e-1 * t37903 - 0.23804984598836975486e-2 * t37905 + 0.21831846657716620896e-2 * t39801 + 0.86682217400542685632e-1 * t39804 + 0.13002332610081402845e0 * t39807;
    let t39814 = t10781 * t7535;
    let t39816 = t980 * t10757;
    (t39809, t39814, t39816)
}
