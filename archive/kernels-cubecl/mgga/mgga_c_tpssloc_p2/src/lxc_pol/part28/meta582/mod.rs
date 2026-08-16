//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1869;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta582<F: Float>(t232: F, t46693: F, t6605: F, t815: F, t2628: F, t58345: F, t2632: F, t47262: F, t22996: F, t6590: F, t25130: F, t828: F, t9627: F, t22986: F, t25249: F, t2679: F, t6646: F, t23110: F, t25299: F, t81651: F, t23168: F, t25313: F, t25319: F, t2553: F, t6552: F, t6637: F) -> (F, F, F, F, F, F, F, F) {
        let (t87495, t87498, t87502, t87507) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1869::<F>(t232, t46693, t6605, t815, t2628, t58345, t2632, t47262, t22996, t6590, t25130, t828, t9627);
        let (t87517, t87520, t87522, t87527) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1870::<F>(t22986, t25249, t2679, t6646, t23110, t25299, t81651, t23168, t25313, t25319, t2553, t6552, t6637);
    (t87495, t87498, t87502, t87507, t87517, t87520, t87522, t87527)
}
