//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1234/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1234<F: Float>(t5100: F, t6271: F, t2236: F, t6508: F, t2252: F, t549: F, t551: F, t6343: F, t1234: F, t566: F, t20455: F, t538: F, t6155: F, t2147: F, t6398: F, t6402: F) -> (F, F, F, F, F, F) {
    let t22883 = t5100 * t6271;
    let t22923 = t2236 * t6508;
    let t22927 = t549 * t551 * t6343 * t2252;
    let t22931 = t566 * t551 * t6343 * t1234;
    let t22939 = t6155 * t538 * t20455;
    let t22942 = t2147 * t6398 * t6402;
    (t22883, t22923, t22927, t22931, t22939, t22942)
}
