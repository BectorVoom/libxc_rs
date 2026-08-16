//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta122 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk607;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk608;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk609;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta122<F: Float>(t1032: F, t989: F, t1040: F, t1024: F, t1062: F, t1031: F, t196: F, t342: F, t1034: F, t358: F, t360: F, t368: F, t335: F, t365: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3124, t3127) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk607::<F>(t1032, t989, t1040, t1024, t1062);
        let t3140 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk608::<F>(t1031, t196);
        let (t3141, t3143, t3144, t3145, t3147, t3148, t3149, t3150, t3153) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk609::<F>(t3140, t342, t1034, t358, t360, t368, t335, t365, t73);
    (t3124, t3127, t3140, t3141, t3143, t3144, t3145, t3147, t3148, t3149, t3150, t3153)
}
