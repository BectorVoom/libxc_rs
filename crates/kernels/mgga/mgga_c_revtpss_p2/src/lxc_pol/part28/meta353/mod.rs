//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta353<F: Float>(t1063: F, t11988: F, t1062: F, t3196: F, t3223: F, t3229: F, t369: F, t361: F, t351: F, t3106: F, t3111: F, t3156: F, t3172: F) -> (F, F, F, F, F, F, F) {
        let (t11989, t11991, t11994, t12002, t12004, t12007, t12009) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1373::<F>(t1063, t11988, t1062, t3196, t3223, t3229, t369, t361, t351, t3106, t3111, t3156, t3172);
    (t11989, t11991, t11994, t12002, t12004, t12007, t12009)
}
