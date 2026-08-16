//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta592<F: Float>(t98968: F, t98972: F, t98983: F, t98991: F, t99000: F, t99006: F, t99011: F, t99019: F, t99021: F, t99023: F, t99026: F, t99029: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t103265, t103267, t103273, t103276, t103280, t103283, t103286, t103290, t103291, t103292, t103293, t103294) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1924::<F>(t98968, t98972, t98983, t98991, t99000, t99006, t99011, t99019, t99021, t99023, t99026, t99029);
    (t103265, t103267, t103273, t103276, t103280, t103283, t103286, t103290, t103291, t103292, t103293, t103294)
}
