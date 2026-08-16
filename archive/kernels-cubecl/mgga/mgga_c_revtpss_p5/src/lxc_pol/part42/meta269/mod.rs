//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1017;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1018;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta269<F: Float>(t1376: F, t9789: F, t235: F, t4086: F, t2453: F, t240: F, t2712: F, t3994: F, t2713: F, t3951: F, t3964: F, t785: F, t9731: F, t225: F, t4062: F, t1386: F, t2482: F, t814: F, t136: F, t1412: F, t220: F, t1353: F, t4003: F, t2735: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9791, t9793, t9794, t9796, t9799, t9801) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1017::<F>(t1376, t9789, t235, t4086, t2453, t240, t2712, t3994, t2713, t3951, t3964, t785, t9731);
        let (t9802, t9804, t9816, t9818, t9835, t9845) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1018::<F>(t225, t9801, t4062, t1386, t2482, t814, t136, t1412, t220, t1353, t4003, t2735, t4086);
    (t9791, t9793, t9794, t9796, t9799, t9802, t9804, t9816, t9818, t9835, t9845)
}
