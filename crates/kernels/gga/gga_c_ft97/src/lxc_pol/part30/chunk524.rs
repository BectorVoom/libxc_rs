//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 524/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk524<F: Float>(t24543: F, t6121: F, t2: F, t6061: F, t1882: F, t6177: F, t1449: F, t668: F, t6081: F, t6090: F, t6187: F, t761: F, t6150: F, t681: F, t89: F, t6168: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24544 = t24543 * t6121;
    let t24546 = t2 * t6061;
    let t24567 = t1882 * t6177;
    let t24569 = t1449 * t668;
    let t24590 = t1882 * t6081;
    let t24592 = t1882 * t6090;
    let t24599 = t761 * t6187;
    let t24605 = t89 * t681 * t6150;
    let t24611 = t1882 * t6168;
    (t24544, t24546, t24567, t24569, t24590, t24592, t24599, t24605, t24611)
}
