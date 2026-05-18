//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1085/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1085<F: Float>(t1181: F, t5567: F, t7351: F, t7426: F, t30934: F, t9608: F, t2001: F, t5529: F, t25941: F, t599: F, t7337: F, t1815: F, t372: F) -> (F, F, F, F, F) {
    let t38755 = t7426 * t1181 * t7351 * t5567;
    let t38757 = t30934 * t9608;
    let t38760 = t2001 * t5529;
    let t38764 = t7337 * t1181 * t599 * t25941;
    let t38766 = t1815 * t372;
    (t38755, t38757, t38760, t38764, t38766)
}
