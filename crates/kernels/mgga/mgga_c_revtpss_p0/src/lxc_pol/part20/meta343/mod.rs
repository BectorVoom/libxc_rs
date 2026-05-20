//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1270;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta343<F: Float>(t16409: F, t342: F, t1071: F, t3316: F, t4980: F, t989: F, t4995: F, t12166: F, t378: F, t11631: F, t12050: F, t12077: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16410, t16505, t16506, t16520, t16523, t16551, t16552, t16553, t16558) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1270::<F>(t16409, t342, t1071, t3316, t4980, t989, t4995, t12166, t378, t11631, t12050, t12077);
    (t16410, t16505, t16506, t16520, t16523, t16551, t16552, t16553, t16558)
}
