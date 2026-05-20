//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta148 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk676;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta148<F: Float>(t1425: F, t560: F, t225: F, t1429: F, t2435: F, t1428: F, t2777: F, t2439: F, t1385: F) -> (F, F, F, F, F, F) {
        let (t4075, t4076, t4082, t4083, t4085, t4086) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk676::<F>(t1425, t560, t225, t1429, t2435, t1428, t2777, t2439, t1385);
    (t4075, t4076, t4082, t4083, t4085, t4086)
}
