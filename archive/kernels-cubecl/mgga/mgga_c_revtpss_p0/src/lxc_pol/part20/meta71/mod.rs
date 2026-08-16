//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta71 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk463;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta71<F: Float>(t1455: F, t117: F, t670: F, t572: F, t573: F, t76: F, t84: F, t198: F, t207: F, t159: F, t215: F, t10: F, t17: F, param_d: F) -> (F, F, F, F, F, F, F) {
        let (t1459, t1461, t1464, t1927, t1940, t1941, t2219) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk463::<F>(t1455, t117, t670, t572, t573, t76, t84, t198, t207, t159, t215, t10, t17, param_d);
    (t1459, t1461, t1464, t1927, t1940, t1941, t2219)
}
