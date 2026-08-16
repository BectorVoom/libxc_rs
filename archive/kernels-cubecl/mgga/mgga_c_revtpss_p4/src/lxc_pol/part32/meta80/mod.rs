//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta80 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk495;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk496;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk497;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk498;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk499;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta80<F: Float>(t1610: F, t915: F, t1594: F, t939: F, t1601: F, t1604: F, t1607: F, t948: F, t951: F, t954: F, t958: F, t324: F, t967: F, t970: F, t973: F, t1598: F, t300: F, t311: F, t946: F, t965: F, t964: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1612, t1614, t1621, t1622) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk495::<F>(t1610, t915, t1594, t939, t1601, t1604, t1607, t948, t951, t954);
        let t1626 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk496::<F>(t1594, t958);
        let (t1627, t1633) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk497::<F>(t1626, t324, t1594, t1601, t1604, t1607, t967, t970);
        let t1634 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk498::<F>(t1633, t973);
        let (t1638, t1640, t1642) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk499::<F>(t1598, t1612, t1614, t1622, t1627, t1634, t300, t311, t946, t965, t1633, t964, t973);
    (t1612, t1614, t1621, t1622, t1626, t1633, t1634, t1638, t1640, t1642)
}
