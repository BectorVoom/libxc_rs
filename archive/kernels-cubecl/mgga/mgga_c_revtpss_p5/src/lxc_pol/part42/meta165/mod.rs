//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta165 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk712;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta165<F: Float>(t2770: F, t4486: F, t1558: F, t251: F, t231: F, t2783: F, t2782: F, t1559: F, t72: F, t686: F, t2798: F, t225: F, t2718: F) -> (F, F, F, F, F, F, F, F) {
        let (t4487, t4494, t4496, t4497, t4499, t4500, t4501, t4503) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk712::<F>(t2770, t4486, t1558, t251, t231, t2783, t2782, t1559, t72, t686, t2798, t225, t2718);
    (t4487, t4494, t4496, t4497, t4499, t4500, t4501, t4503)
}
