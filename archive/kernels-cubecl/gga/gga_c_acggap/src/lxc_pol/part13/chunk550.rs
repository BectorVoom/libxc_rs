//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 550/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk550<F: Float>(t1036: F, t3657: F, t1165: F, t388: F, t955: F, t1163: F, t134: F, t972: F, t161: F, t151: F) -> (F, F, F, F, F, F) {
    let t3658 = t1036 * t3657;
    let t3665 = t1165 * t388 * t955;
    let t3666 = t1163 * t3665;
    let t3668 = t972 * t134;
    let t3669 = t161 * t3668;
    let t3670 = t151 * t3669;
    (t3658, t3665, t3666, t3668, t3669, t3670)
}
