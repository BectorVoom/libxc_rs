//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta968 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3233;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta968<F: Float>(t50852: F, t50856: F, t18562: F, t2516: F, t2496: F, t18305: F, t2258: F, t4401: F, t14325: F, t18306: F, t5825: F, t749: F) -> (F, F, F, F, F, F, F) {
        let (t61292, t61293, t61295, t61297, t61300, t61302, t61303) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3233::<F>(t50852, t50856, t18562, t2516, t2496, t18305, t2258, t4401, t14325, t18306, t5825, t749);
    (t61292, t61293, t61295, t61297, t61300, t61302, t61303)
}
