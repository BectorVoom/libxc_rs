//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1933;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1934;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta582<F: Float>(t11064: F, t1113: F, t27384: F, t27799: F, t98767: F, t33: F, t41154: F, t98786: F, t1711: F, t2411: F, t14365: F, t1544: F, t3351: F, t4343: F, t1583: F, t63164: F, t4433: F, t892: F, t14749: F, t27763: F, t14767: F, t2408: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t100975, t100978, t100982, t100988, t100993) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1933::<F>(t11064, t1113, t27384, t27799, t98767, t33, t41154, t98786, t1711, t2411, t14365, t1544, t3351);
        let (t100997, t101012, t101016, t101029, t101032, t101035, t101040) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1934::<F>(t1113, t4343, t1583, t3351, t27799, t63164, t4433, t892, t14749, t27763, t14767, t1711, t2408);
    (t100975, t100978, t100982, t100988, t100993, t100997, t101012, t101016, t101029, t101032, t101035, t101040)
}
