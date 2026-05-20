//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1961;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta620<F: Float>(t22068: F, t25972: F, t25978: F, t6880: F, t6856: F, t1398: F, t543: F, t6895: F, t1907: F, t5591: F, t5778: F, t5920: F, t648: F) -> (F, F, F, F, F, F, F) {
        let (t108625, t108627, t108629, t108653, t108682, t108688, t108710) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1961::<F>(t22068, t25972, t25978, t6880, t6856, t1398, t543, t6895, t1907, t5591, t5778, t5920, t648);
    (t108625, t108627, t108629, t108653, t108682, t108688, t108710)
}
