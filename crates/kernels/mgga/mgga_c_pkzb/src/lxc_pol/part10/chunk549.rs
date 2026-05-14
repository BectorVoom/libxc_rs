//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 549/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk549<F: Float>(t2173: F, t2175: F, t2187: F, t352: F, t828: F, t832: F) -> (F, F, F) {
    let t2189 = t2173 - 0.35616666666666666666e-1 * t2175 + 0.53425e-1 * t2187;
    let t2191 = 0.621814e-1 * t2189 * t352;
    let t2192 = t828 * t832;
    (t2189, t2191, t2192)
}
