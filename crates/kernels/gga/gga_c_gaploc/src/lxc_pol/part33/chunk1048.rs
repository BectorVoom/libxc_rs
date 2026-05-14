//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1048/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1048<F: Float>(t31590: F, t6508: F, t1358: F, t6507: F, t10269: F, t3808: F, t29896: F, t29898: F, t29901: F, t29903: F, t29908: F, t29911: F, t29913: F, t29915: F, t471: F, t10205: F, t64: F) -> (F, F, F, F, F) {
    let t31591 = t6508 * t31590;
    let t31594 = 0.12646669615856066488e-1 * t1358 * t6507 * t31591;
    let t31600 = 0.12646669615856066488e-1 * t3808 * t10269;
    let t31610 = (189.0 / 512.0 * t29896 - 2499.0 / 16384.0 * t29898 + 1239.0 / 524288.0 * t29901 - 441.0 / 0.16777216e8 * t29903 + 147.0 / 0.16777216e8 * t29908 - 413.0 / 524288.0 * t29911 + 833.0 / 16384.0 * t29913 - 63.0 / 512.0 * t29915) * t471;
    let t31612 = 8.0 / 3.0 * t10205 * t64;
    (t31591, t31594, t31600, t31610, t31612)
}
