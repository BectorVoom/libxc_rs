//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1771;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta475<F: Float>(t1874: F, t19456: F, t4028: F, t6525: F, t5161: F, t6996: F, t1983: F, t1914: F, t193: F, t200: F) -> (F, F, F, F, F) {
        let (t25005, t25007, t25010, t25011, t25013) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1771::<F>(t1874, t19456, t4028, t6525, t5161, t6996, t1983, t1914, t193, t200);
    (t25005, t25007, t25010, t25011, t25013)
}
