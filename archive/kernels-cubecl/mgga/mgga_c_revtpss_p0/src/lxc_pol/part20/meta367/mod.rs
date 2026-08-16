//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta367 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1340;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1341;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta367<F: Float>(t10326: F, t706: F, t750: F, t2523: F, t9419: F, t40093: F, t40095: F, t40099: F, t40103: F, t40106: F, t40109: F, t40111: F, t40115: F, t40117: F, t10558: F, t72: F, t757: F, t10573: F, t2619: F, t2598: F, t9321: F, t760: F, t9387: F, t2495: F, t39875: F, t9367: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40120, t40122, t40123) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1340::<F>(t10326, t706, t750, t2523, t9419, t40093, t40095, t40099, t40103, t40106, t40109, t40111, t40115, t40117);
        let (t40126, t40128, t40129, t40131, t40133, t40135) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1341::<F>(t10558, t72, t757, t10573, t2619, t2598, t9321, t760, t2523, t9387, t2495, t39875, t9367);
    (t40120, t40122, t40123, t40126, t40128, t40129, t40131, t40133, t40135)
}
