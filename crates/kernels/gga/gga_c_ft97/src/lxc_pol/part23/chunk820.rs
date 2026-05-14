//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 820/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk820<F: Float>(t1882: F, t6081: F, t6090: F, t6187: F, t761: F, t6150: F, t681: F, t89: F, t6168: F, t24482: F, t24537: F, t1445: F, t2399: F, t1449: F, t2567: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24590 = t1882 * t6081;
    let t24592 = t1882 * t6090;
    let t24599 = t761 * t6187;
    let t24605 = t89 * t681 * t6150;
    let t24611 = t1882 * t6168;
    let t24628 = 4.0 / 27.0 * t24482;
    let t24642 = 2.0 / 27.0 * t24537;
    let t24658 = 4.0 / 27.0 * t89 * t2399 * t1445;
    let t24668 = t2567 * t1449;
    (t24590, t24592, t24599, t24605, t24611, t24628, t24642, t24658, t24668)
}
