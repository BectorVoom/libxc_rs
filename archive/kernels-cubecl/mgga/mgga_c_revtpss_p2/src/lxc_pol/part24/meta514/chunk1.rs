//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1533/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1533<F: Float>(t1668: F, t905: F, t11774: F, t53391: F, t6267: F, t19968: F, t4817: F, t20054: F, t4834: F, t19882: F, t1062: F, t23960: F) -> (F, F, F, F, F, F) {
    let t79450 = t1668 * t905;
    let t79474 = t11774 * t53391 * t6267;
    let t79546 = t19968 * t4817;
    let t79548 = t4834 * t20054;
    let t79553 = t4834 * t19882;
    let t79559 = t23960 * t1062;
    (t79450, t79474, t79546, t79548, t79553, t79559)
}
