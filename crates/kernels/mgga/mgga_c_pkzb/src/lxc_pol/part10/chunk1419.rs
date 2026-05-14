//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1419/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1419<F: Float>(t28628: F, t6751: F, t6752: F, t6754: F, t7218: F, t7219: F, t7917: F, t8710: F, t8711: F, t8713: F, t9: F, t9746: F, t9748: F, t23617: F) -> (F,) {
    let t28633 = 2.0 * t7218 + 4.0 * t8713 - 0.286875e0 * t7219 + 4.0 * t8711 + 2.0 * t8710 + 0.19125e0 * t9748 - 0.286875e0 * t9746 + 0.95625e-1 * t7917 + t9 * t28628 + 2.0 * t6754 + 2.0 * t6751 + 4.0 * t6752;
    let tv4rho42 = t23617 + t28633;
    (tv4rho42,)
}
