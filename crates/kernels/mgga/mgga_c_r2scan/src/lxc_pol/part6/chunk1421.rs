//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1421/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1421<F: Float>(t22232: F, t7755: F, t22003: F, t22010: F, t22012: F, t22534: F, t22537: F, t22542: F, t22546: F, t22550: F, t22554: F, t22557: F, t22560: F, t22569: F, t22574: F, t410: F, t7794: F) -> (F, F) {
    let t26849 = t7755 * t22232;
    let t26852 = -t22534 + t22537 + t22542 + t22546 + t22550 - t22554 + 0.60030643514799999998e-2 * t26849 + t22003 + t22557 + t22560 + 4.0 * t22569 - t22574 - t22010 + t22012;
    let t26860 = t410 * t7794;
    (t26852, t26860)
}
