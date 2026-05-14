//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 591/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk591<F: Float>(t1025: F, t1702: F, t1024: F, t568: F, t581: F, t2575: F, t50: F, t168: F, t1717: F) -> (F, F, F, F, F) {
    let t2580 = t1702 * t1025;
    let t2583 = t581 * t1024 * t568;
    let t2586 = t50 * t2575;
    let t2587 = t581 * t2586;
    let t2590 = t1717 * t168;
    (t2580, t2583, t2586, t2587, t2590)
}
