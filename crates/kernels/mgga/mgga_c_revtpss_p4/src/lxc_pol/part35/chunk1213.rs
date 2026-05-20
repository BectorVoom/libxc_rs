//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1213/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1213<F: Float>(t110177: F, t113111: F, t113420: F, t113424: F, t113444: F, t113454: F, t113461: F, t113465: F, t113484: F, t1468: F, t1940: F, t2071: F, t22670: F, t2403: F, t26590: F, t28460: F, t29716: F, t29719: F, t30420: F, t4541: F, t5824: F, t7432: F, t7749: F, t7787: F, t8020: F, t95964: F) -> F {
    let t115462 = F::new(9.0) / F::new(2.0) * t2403 * t2071 * t113461 + F::new(9.0) / F::new(2.0) * t2403 * t2071 * t113420 + F::new(9.0) * t4541 * t2071 * t113454 + F::new(3.0) * t1940 * t26590 * t113465 - t1940 * t7432 * t113424 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t1940 * t110177 * t7787 - F::new(3.0) / F::new(2.0) * t1940 * t28460 * t29719 - F::new(3.0) * t1940 * t95964 * t113444 + F::new(9.0) / F::new(2.0) * t2403 * t30420 * t7749 - F::new(3.0) / F::new(2.0) * t1940 * t7432 * t113484 - F::new(3.0) * t1940 * t28460 * t29716 + F::new(3.0) / F::new(2.0) * t1940 * t8020 * t5824 + t1940 * t2071 * t22670 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t1940 * t7432 * t113111 + F::new(3.0) / F::new(2.0) * t1940 * t30420 * t1468;
    t115462
}
