//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1850;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta524<F: Float>(t94564: F, t9795: F, t2018: F, t40688: F, t46808: F, t7256: F, t9784: F, t25877: F, t94390: F, t1032: F, t4066: F, t1955: F, t1399: F, t2434: F, t3924: F, t676: F, t46361: F, t545: F, t9656: F, t25875: F, t25894: F, t7282: F, t9646: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94565, t94568, t94570, t94589, t94609, t94610) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1850::<F>(t94564, t9795, t2018, t40688, t46808, t7256, t9784, t25877, t94390, t1032, t4066, t1955);
        let (t94633, t94639, t94656, t94669, t94674, t94696) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1851::<F>(t1399, t2434, t3924, t676, t46361, t545, t1032, t9656, t25875, t25894, t7282, t9646);
    (t94565, t94568, t94570, t94589, t94609, t94610, t94633, t94639, t94656, t94669, t94674, t94696)
}
