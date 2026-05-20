//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta256 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta256<F: Float>(t16543: F, t994: F, t12166: F, t378: F, t342: F, t11631: F, t12050: F, t12077: F, t3154: F, t12046: F, t1647: F, t3316: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t16544, t16551, t16552, t16553, t16558, t16559, t16560, t16565, t16566, t16584) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1024::<F>(t16543, t994, t12166, t378, t342, t11631, t12050, t12077, t3154, t12046, t1647, t3316);
    (t16544, t16551, t16552, t16553, t16558, t16559, t16560, t16565, t16566, t16584)
}
