//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1271;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta344<F: Float>(t16558: F, t342: F, t12050: F, t3154: F, t3151: F, t12046: F, t378: F, t357: F, t379: F, t994: F, t1214: F, t5333: F) -> (F, F, F, F, F, F, F) {
        let (t16559, t16561, t16565, t16566, t16568, t16603, t16696) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1271::<F>(t16558, t342, t12050, t3154, t3151, t12046, t378, t357, t379, t994, t1214, t5333);
    (t16559, t16561, t16565, t16566, t16568, t16603, t16696)
}
