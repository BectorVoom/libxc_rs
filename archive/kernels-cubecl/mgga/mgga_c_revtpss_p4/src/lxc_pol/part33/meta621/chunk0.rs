//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2060/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2060<F: Float>(t25081: F, t7897: F, t2033: F, t47672: F, t2: F, t2411: F, t198: F, t206: F, t7782: F, t892: F, t1468: F, t11064: F) -> (F, F, F, F, F, F, F) {
    let t98450 = t7897 * t25081;
    let t98495 = t2033 * t47672;
    let t98631 = t2411 * t2;
    let t98637 = t198 * t206 * t7782;
    let t98646 = t892 * t2;
    let t98658 = t2411 * t1468;
    let t98722 = t7782 * t11064;
    (t98450, t98495, t98631, t98637, t98646, t98658, t98722)
}
