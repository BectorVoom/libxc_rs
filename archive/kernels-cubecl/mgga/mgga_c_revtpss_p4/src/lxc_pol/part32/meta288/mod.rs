//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta288 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1186;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta288<F: Float>(t1376: F, t9789: F, t235: F, t4086: F, t2453: F, t240: F, t2712: F, t3994: F, t2713: F, t3951: F, t3964: F, t785: F, t9731: F) -> (F, F, F, F, F, F, F) {
        let (t9791, t9793, t9794, t9795, t9796, t9799, t9801) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1186::<F>(t1376, t9789, t235, t4086, t2453, t240, t2712, t3994, t2713, t3951, t3964, t785, t9731);
    (t9791, t9793, t9794, t9795, t9796, t9799, t9801)
}
