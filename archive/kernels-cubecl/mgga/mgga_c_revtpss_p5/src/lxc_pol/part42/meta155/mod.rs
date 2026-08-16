//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta155 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk692;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta155<F: Float>(t4292: F, t508: F, t1843: F, t670: F, t2616: F, t2524: F, t1534: F, t72: F, t757: F, t1469: F, t750: F, t706: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4293, t4297, t4300, t4301, t4302, t4303, t4304, t4305, t4306) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk692::<F>(t4292, t508, t1843, t670, t2616, t2524, t1534, t72, t757, t1469, t750, t706);
    (t4293, t4297, t4300, t4301, t4302, t4303, t4304, t4305, t4306)
}
