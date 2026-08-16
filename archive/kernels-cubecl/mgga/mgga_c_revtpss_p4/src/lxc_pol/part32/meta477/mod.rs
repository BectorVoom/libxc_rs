//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1714;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta477<F: Float>(t27212: F, t786: F, t7063: F, t14685: F, t1941: F, t14756: F, t4435: F, t7045: F, t4426: F, t7038: F, t25245: F, t4430: F) -> (F, F, F, F, F, F, F) {
        let (t27213, t27216, t27221, t27222, t27224, t27226, t27228) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1714::<F>(t27212, t786, t7063, t14685, t1941, t14756, t4435, t7045, t4426, t7038, t25245, t4430);
    (t27213, t27216, t27221, t27222, t27224, t27226, t27228)
}
