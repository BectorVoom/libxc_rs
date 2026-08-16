//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1501/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1501<F: Float>(t14598: F, t23160: F, t686: F, t72: F, t23244: F, t251: F, t1568: F, t5977: F, t2723: F, t2782: F, t4503: F, t1558: F, t6041: F) -> (F, F, F, F, F) {
    let t76125 = t14598 * t23160 * t72 * t686;
    let t76127 = t251 * t23244;
    let t76131 = t1568 * t5977;
    let t76134 = t2782 * t4503 * t76131 * t2723;
    let t76136 = t6041 * t1558;
    (t76125, t76127, t76131, t76134, t76136)
}
