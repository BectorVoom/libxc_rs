//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 777/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk777<F: Float>(t1424: F, t2569: F, t2568: F, t729: F, t6187: F, t761: F, t684: F, t2606: F, t6150: F, t681: F, t89: F, t1456: F, t2409: F, t724: F, t1882: F, t6168: F) -> (F, F, F, F, F, F, F, F) {
    let t24594 = t1424 * t2569;
    let t24596 = t729 * t2568 * t24594;
    let t24599 = t761 * t6187;
    let t24600 = t24599 * t684;
    let t24601 = t2606 * t24600;
    let t24605 = t89 * t681 * t6150;
    let t24608 = t724 * t1456 * t2409;
    let t24611 = t1882 * t6168;
    (t24594, t24596, t24599, t24600, t24601, t24605, t24608, t24611)
}
