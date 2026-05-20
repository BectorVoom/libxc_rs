//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta730 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2499;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta730<F: Float>(t49957: F, t14322: F, t2496: F, t2609: F, t4186: F, t706: F, t14616: F, t2619: F, t198: F, t775: F, t10565: F, t1469: F) -> (F, F, F, F, F, F) {
        let (t49958, t49964, t49982, t50048, t50080, t50084) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2499::<F>(t49957, t14322, t2496, t2609, t4186, t706, t14616, t2619, t198, t775, t10565, t1469);
    (t49958, t49964, t49982, t50048, t50080, t50084)
}
