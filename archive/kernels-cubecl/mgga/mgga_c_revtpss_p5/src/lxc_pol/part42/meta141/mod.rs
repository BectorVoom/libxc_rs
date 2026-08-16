//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta141 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk659;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta141<F: Float>(t1419: F, t212: F, t1358: F, t689: F, t1357: F, t1445: F, t2453: F, t556: F, t136: F, t561: F, t2457: F, t1420: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3899, t3900, t3901, t3903, t3904, t3906, t3907, t3908, t3910, t3911) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk659::<F>(t1419, t212, t1358, t689, t1357, t1445, t2453, t556, t136, t561, t2457, t1420, t786);
    (t3899, t3900, t3901, t3903, t3904, t3906, t3907, t3908, t3910, t3911)
}
