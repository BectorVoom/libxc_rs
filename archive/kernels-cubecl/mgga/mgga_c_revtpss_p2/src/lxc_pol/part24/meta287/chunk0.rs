//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1067/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1067<F: Float>(t19826: F, t3161: F, t1058: F, t6318: F, t1062: F, t15670: F, t247: F, t3109: F, t6096: F, t1063: F, t140: F, t6284: F) -> (F, F, F, F, F, F) {
    let t19827 = t3161 * t19826;
    let t19867 = t6318 * t1058;
    let t19878 = t15670 * t1062;
    let t19882 = t247 * t3109 * t6096;
    let t19883 = t1063 * t19882;
    let t19900 = t140 * t6284;
    (t19827, t19867, t19878, t19882, t19883, t19900)
}
