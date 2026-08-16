//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta553<F: Float>(t28107: F, t553: F, t1998: F, t6434: F, t214: F, t1985: F, t19739: F, t550: F, t6976: F, t1992: F, t19660: F, t22709: F, t6388: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28156, t28159, t28160, t28161, t28163, t28164, t28165, t28167, t28168, t28169, t28171) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1911::<F>(t28107, t553, t1998, t6434, t214, t1985, t19739, t550, t6976, t1992, t19660, t22709, t6388);
    (t28156, t28159, t28160, t28161, t28163, t28164, t28165, t28167, t28168, t28169, t28171)
}
