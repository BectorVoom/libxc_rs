//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1111/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1111<F: Float>(t1878: F, t209: F, t81982: F, t6604: F, t9971: F, t206: F, t22723: F, t268: F, t23163: F, t1879: F, t80845: F, t1906: F) -> (F, F, F, F, F, F) {
    let t81984 = t1878 * t81982 * t209;
    let t82018 = t6604 * t9971;
    let t82031 = t22723 * t206 * t268;
    let t82038 = t22723 * t23163;
    let t82045 = t80845 * t1879;
    let t82046 = t82045 * t1906;
    (t81984, t82018, t82031, t82038, t82045, t82046)
}
