//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2913/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2913<F: Float>(t3926: F, t9909: F, t9775: F, t9981: F, t1389: F, t3964: F, t40604: F, t3961: F, t9741: F, t10111: F, t22: F, t4092: F) -> (F, F, F, F, F) {
    let t47304 = t9909 * t3926;
    let t47320 = t9775 * t9981;
    let t47337 = F::cast_from(0.11344944493805280483e-2_f64) * t3964 * t40604 * t1389;
    let t47338 = t9741 * t3961;
    let t47348 = t10111 * t4092 * t22;
    (t47304, t47320, t47337, t47338, t47348)
}
