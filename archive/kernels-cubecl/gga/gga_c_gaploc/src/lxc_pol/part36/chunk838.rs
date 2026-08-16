//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 838/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk838<F: Float>(t41809: F, t447: F, t6963: F, t6964: F, t12915: F, t4953: F, t1445: F, t1562: F, t34202: F, t874: F, t34157: F, t895: F) -> (F, F, F, F, F) {
    let t41810 = t41809 * t447;
    let t41813 = F::cast_from(0.71500979903700853338e0_f64) * t6963 * t6964 * t41810;
    let t41814 = t4953 * t12915;
    let t41818 = t1562 * t1445 * t34202 * t874;
    let t41820 = t895 * t34157;
    (t41810, t41813, t41814, t41818, t41820)
}
