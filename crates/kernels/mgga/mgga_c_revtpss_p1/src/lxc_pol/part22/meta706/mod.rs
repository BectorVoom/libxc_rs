//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta706 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2729;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta706<F: Float>(t21660: F, t22531: F, t3: F, t5883: F, t670: F, t4292: F, t5801: F, t116: F, t5920: F, t117: F, t21881: F, t1459: F, t1461: F, t1916: F, t1918: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t6941: F, t6945: F, t6948: F, param_d: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t22532, t22533, t22544, t22556, t22559, t22564, t22565, t22568, t22571) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2729::<F>(t21660, t22531, t3, t5883, t670, t4292, t5801, t116, t5920, t117, t21881, t1459, t1461, t1916, t1918, t572, t573, t5795, t5802, t5805, t6941, t6945, t6948, param_d);
    (t22532, t22533, t22544, t22556, t22559, t22564, t22565, t22568, t22571)
}
