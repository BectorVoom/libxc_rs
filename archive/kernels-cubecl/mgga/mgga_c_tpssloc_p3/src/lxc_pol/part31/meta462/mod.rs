//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1616;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta462<F: Float>(t1888: F, t25303: F, t1484: F, t23153: F, t6637: F, t6552: F, t23168: F, t7521: F, t4119: F, t6638: F, t22893: F, t7520: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25304, t25306, t25307, t25308, t25310, t25312, t25313, t25314, t25316) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1616::<F>(t1888, t25303, t1484, t23153, t6637, t6552, t23168, t7521, t4119, t6638, t22893, t7520);
    (t25304, t25306, t25307, t25308, t25310, t25312, t25313, t25314, t25316)
}
