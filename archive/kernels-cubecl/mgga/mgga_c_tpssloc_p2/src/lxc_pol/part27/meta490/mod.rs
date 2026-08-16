//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1875;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta490<F: Float>(t5308: F, t8945: F, t24995: F, t111: F, t7450: F, t1874: F, t19456: F, t4028: F, t6525: F, t5161: F, t6996: F, t1983: F, t1914: F, t193: F, t200: F) -> (F, F, F, F, F, F, F, F) {
        let (t24996, t24998, t24999) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1875::<F>(t5308, t8945, t24995, t111, t7450);
        let (t25005, t25007, t25010, t25011, t25013) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1876::<F>(t1874, t19456, t4028, t6525, t5161, t6996, t1983, t1914, t193, t200);
    (t24996, t24998, t24999, t25005, t25007, t25010, t25011, t25013)
}
