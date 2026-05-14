//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 802/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk802<F: Float>(t1092: F, t3051: F, t1113: F, t2427: F, t1103: F, t2247: F, t228: F, t231: F, t1123: F, t2248: F, t701: F, t1132: F, t2999: F, t89: F, t1152: F, t1148: F, t3139: F) -> (F, F, F, F, F, F, F) {
    let t52453 = t3051 * t1092;
    let t52563 = t2427 * t1113;
    let t52668 = t1103 * t2247;
    let t52670 = t228 * t52668 * t231;
    let t52752 = t701 * t2248 * t1123;
    let t52916 = t89 * t2999 * t1132;
    let t53123 = t3051 * t1152;
    let t53287 = t3139 * t1148;
    (t52453, t52563, t52670, t52752, t52916, t53123, t53287)
}
