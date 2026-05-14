//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1028/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1028<F: Float>(t2030: F, t20559: F, t8919: F, t301: F, t31146: F, t4256: F, t9529: F, t1891: F, t7614: F, t1998: F, t6125: F, t30811: F, t6090: F, t30543: F, t9670: F, t1165: F, t39794: F, t604: F, t7413: F) -> (F, F, F, F, F, F, F) {
    let t40377 = t2030 * t20559 * t8919;
    let t40381 = t31146 * t4256 * t9529 * t301;
    let t40385 = t7614 * t1891;
    let t40387 = t1998 * t6125;
    let t40390 = t30811 * t6090;
    let t40398 = t30543 * t9670;
    let t40403 = t7413 * t1165 * t604 * t39794;
    (t40377, t40381, t40385, t40387, t40390, t40398, t40403)
}
