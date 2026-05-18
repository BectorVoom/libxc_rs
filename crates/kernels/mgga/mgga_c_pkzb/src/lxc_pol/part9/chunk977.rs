//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 977/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk977<F: Float>(t2901: F, t7653: F, t302: F, t2900: F, t6022: F, t5953: F, t7663: F) -> (F, F, F, F, F) {
    let t7728 = t7653 * t2901;
    let t7729 = t302 * t7728;
    let t7732 = t2900 * t6022;
    let t7733 = t302 * t7732;
    let t7736 = t5953 * t7663;
    (t7728, t7729, t7732, t7733, t7736)
}
