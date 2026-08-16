//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta669 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta669<F: Float>(t1086: F, t3259: F, t994: F, t3046: F, t4980: F, t12153: F, t12046: F, t989: F, t1035: F, t42859: F, t342: F, t11902: F) -> (F, F, F, F, F, F, F) {
        let (t43357, t43360, t43378, t43384, t43400, t43401, t43413) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2470::<F>(t1086, t3259, t994, t3046, t4980, t12153, t12046, t989, t1035, t42859, t342, t11902);
    (t43357, t43360, t43378, t43384, t43400, t43401, t43413)
}
