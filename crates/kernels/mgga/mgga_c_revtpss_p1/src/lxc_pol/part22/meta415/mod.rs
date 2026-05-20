//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2020;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta415<F: Float>(t2251: F, t4402: F, t14330: F, t2516: F, t4398: F, t2496: F, t10443: F, t10552: F, t10554: F, t14312: F, t14313: F, t14315: F, t14317: F, t14318: F, t14324: F, t14327: F, t14329: F, t4541: F, t775: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F) -> (F, F, F, F, F, F, F) {
        let (t14331, t14333, t14334, t14335, t14336, t14337, t14338) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2020::<F>(t2251, t4402, t14330, t2516, t4398, t2496, t10443, t10552, t10554, t14312, t14313, t14315, t14317, t14318, t14324, t14327, t14329, t4541, t775, t9278, t9308, t9316, t9329, t9333);
    (t14331, t14333, t14334, t14335, t14336, t14337, t14338)
}
