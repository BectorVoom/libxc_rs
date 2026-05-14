//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1140/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1140<F: Float>(t1640: F, t1881: F, t2233: F, t28316: F, t28877: F, t29665: F, t6888: F, t6895: F, t7886: F, t7889: F, t7892: F, t8015: F, t91885: F, t91895: F, t91901: F, t92157: F, t92379: F, t97561: F) -> (F,) {
    let t101807 = -t91885 - t29665 * t7886 / 8.0 + t97561 + t1881 * t28877 / 8.0 + t91895 - t6888 * t8015 / 8.0 - t6888 * t7892 / 8.0 - t91901 + t92379 + t1881 * t28316 / 8.0 - t29665 * t7889 / 8.0 + t92157 - t2233 * t6895 * t1640 / 16.0;
    (t101807,)
}
