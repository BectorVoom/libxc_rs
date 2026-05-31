//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1410/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1410<F: Float>(t11075: F, t2403: F, t2430: F, t40093: F, t40095: F, t40099: F, t40103: F, t40106: F, t40109: F, t40111: F, t40115: F, t40117: F, t40120: F, t40122: F, t40126: F) -> F {
    let t41174 = F::cast_from(18.0_f64) * t11075 * t2403 * t2430 - t40093 + t40095 + t40099 + t40103 + t40106 - t40109 + t40111 - t40115 + t40117 + t40120 + t40122 - t40126;
    t41174
}
