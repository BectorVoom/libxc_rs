//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta147 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk675;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta147<F: Float>(t1340: F, t2496: F, t1330: F, t177: F, t762: F, t2626: F, t1412: F, t73: F, t1389: F, t1408: F, t2736: F, t1419: F, t213: F) -> (F, F, F, F, F, F, F, F) {
        let (t4037, t4038, t4039, t4042, t4049, t4062, t4064, t4071) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk675::<F>(t1340, t2496, t1330, t177, t762, t2626, t1412, t73, t1389, t1408, t2736, t1419, t213);
    (t4037, t4038, t4039, t4042, t4049, t4062, t4064, t4071)
}
