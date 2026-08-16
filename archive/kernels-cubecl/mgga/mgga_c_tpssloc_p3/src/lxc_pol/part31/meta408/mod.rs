//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta408<F: Float>(t172: F, t6320: F, t763: F, t15972: F, t12097: F, t12106: F, t12111: F, t12103: F, t12105: F, t12109: F, t12114: F, t12116: F, t12118: F, t15976: F, t9793: F, t9797: F, t9820: F, t9824: F) -> (F, F, F, F, F, F) {
        let (t19683, t19684, t19685, t19686, t19687, t19688) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1500::<F>(t172, t6320, t763, t15972, t12097, t12106, t12111, t12103, t12105, t12109, t12114, t12116, t12118, t15976, t9793, t9797, t9820, t9824);
    (t19683, t19684, t19685, t19686, t19687, t19688)
}
