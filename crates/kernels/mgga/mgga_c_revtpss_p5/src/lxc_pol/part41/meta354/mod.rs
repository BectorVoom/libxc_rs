//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta354 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1165;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta354<F: Float>(t1749: F, t3520: F, t16868: F, t16712: F, t16892: F, t16708: F, t3495: F, t1770: F, t3781: F, t1284: F, t1811: F, t1209: F) -> (F, F, F, F, F, F, F, F) {
        let (t17097, t17115, t17117, t17131, t17140, t17154, t17183, t17192) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1165::<F>(t1749, t3520, t16868, t16712, t16892, t16708, t3495, t1770, t3781, t1284, t1811, t1209);
    (t17097, t17115, t17117, t17131, t17140, t17154, t17183, t17192)
}
