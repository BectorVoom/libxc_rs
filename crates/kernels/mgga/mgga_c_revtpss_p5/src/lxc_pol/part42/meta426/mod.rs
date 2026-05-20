//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta426 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1488;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1489;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta426<F: Float>(t116926: F, t8312: F, t116929: F, t8316: F, t10241: F, t104: F, t46089: F, t655: F, t10199: F, t2339: F, t31027: F, t31430: F, t31032: F, t31434: F, t31447: F, t2357: F, t55: F, t8402: F, t8395: F, t2289: F, t8399: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t117184, t117186, t117218, t117461, t117544, t117918) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1488::<F>(t116926, t8312, t116929, t8316, t10241, t104, t46089, t655, t10199, t2339, t31027, t31430);
        let (t117920, t117927, t117932, t117936, t117938, t117940) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1489::<F>(t31032, t31434, t117461, t31447, t2357, t55, t116929, t8402, t116926, t8395, t2289, t8399);
    (t117184, t117186, t117218, t117544, t117918, t117920, t117927, t117932, t117936, t117938, t117940)
}
