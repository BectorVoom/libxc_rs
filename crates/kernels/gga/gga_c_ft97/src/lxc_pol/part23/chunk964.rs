//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 964/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk964<F: Float>(t1501: F, t15133: F, t317: F, t7021: F, t684: F, t2665: F, t25462: F, t6967: F, t1091: F, t25465: F, t25446: F, t1465: F, t3051: F) -> (F, F, F, F, F, F, F) {
    let t28983 = t15133 * t1501;
    let t28985 = t7021 * t317;
    let t28986 = t28985 * t684;
    let t28987 = t2665 * t28986;
    let t28990 = t25462 * t6967;
    let t28992 = t25465 * t1091;
    let t28993 = t2665 * t28992;
    let t28997 = t2665 * t25446 * t1091;
    let t29000 = t1465 * t3051;
    (t28983, t28985, t28987, t28990, t28993, t28997, t29000)
}
