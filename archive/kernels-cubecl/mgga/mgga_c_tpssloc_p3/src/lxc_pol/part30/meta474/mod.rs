//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1769;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1770;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta474<F: Float>(t6997: F, t7685: F, t1390: F, t5187: F, t6878: F, t1983: F, t192: F, t531: F, t1982: F, t5308: F, t8945: F, t111: F, t7450: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t24989, t24990, t24991, t24993, t24994, t24995) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1769::<F>(t6997, t7685, t1390, t5187, t6878, t1983, t192, t531, t1982);
        let (t24996, t24998, t24999) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1770::<F>(t5308, t8945, t24995, t111, t7450);
    (t24989, t24990, t24991, t24993, t24994, t24995, t24996, t24998, t24999)
}
