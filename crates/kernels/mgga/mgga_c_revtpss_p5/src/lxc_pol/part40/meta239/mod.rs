//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta239 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta239<F: Float>(t1010: F, t1480: F, t1715: F, t3634: F, t247: F, t1261: F, t1260: F, t1785: F, t3670: F, t3719: F, t5230: F, t1802: F, t369: F) -> (F, F, F, F, F, F, F) {
        let (t5373, t5378, t5379, t5381, t5384, t5386, t5389) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk916::<F>(t1010, t1480, t1715, t3634, t247, t1261, t1260, t1785, t3670, t3719, t5230, t1802, t369);
    (t5373, t5378, t5379, t5381, t5384, t5386, t5389)
}
