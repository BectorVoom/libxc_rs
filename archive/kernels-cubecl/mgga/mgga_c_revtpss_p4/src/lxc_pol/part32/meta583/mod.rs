//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta583<F: Float>(t98229: F, t98235: F, t98238: F, t98243: F, t98258: F, t98269: F, t98281: F, t1904: F, t2439: F, t26358: F, t213: F, t28888: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t102531, t102534, t102535, t102537, t102548, t102557, t102567, t102582, t102594) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1911::<F>(t98229, t98235, t98238, t98243, t98258, t98269, t98281, t1904, t2439, t26358, t213, t28888);
    (t102531, t102534, t102535, t102537, t102548, t102557, t102567, t102582, t102594)
}
