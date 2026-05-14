//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 456/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk456<F: Float>(t60: F, t116: F, t3042: F, t114: F, t126: F, t923: F, t6: F, t927: F, t123: F, t2925: F) -> (F, F, F, F, F, F) {
    let t124 = 0.0 < t60;
    let t3043 = t116 * t3042;
    let t3044 = t114 * t3043;
    let t3050 = 1.0 / t923 / t126;
    let t3051 = t6 * t3050;
    let t3052 = t927 * t927;
    let t3054 = t123 * t3051 * t3052;
    let t3058 = piecewise3(t124, t2925, -t2925);
    (t3043, t3044, t3050, t3052, t3054, t3058)
}
