//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 853/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk853<F: Float>(t14575: F, t7942: F, t8306: F, t8111: F, t880: F, t32194: F, t7963: F, t2176: F, t3912: F, t2132: F, t2217: F, t7885: F, t864: F, t29976: F, t8337: F, t29979: F, t29980: F, t638: F) -> (F, F, F, F, F, F, F) {
    let t33097 = t7942 * t8306 * t14575;
    let t33100 = 0.19756347548806534796e1 * t8111 * t880;
    let t33104 = t7963 * t8306 * t32194;
    let t33107 = 0.65854491829355115987e0 * t2176 * t3912;
    let t33118 = t7885 * t2132 * t2217 * t864;
    let t33120 = t29976 * t8337;
    let t33150 = t29979 * t638 * t29980;
    (t33097, t33100, t33104, t33107, t33118, t33120, t33150)
}
