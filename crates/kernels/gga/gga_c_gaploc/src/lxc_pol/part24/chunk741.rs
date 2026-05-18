//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 741/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk741<F: Float>(t6706: F, t6964: F, t2471: F, t4399: F, t1305: F, t487: F, t2365: F, t1416: F, t4803: F, t586: F, t2479: F, t1065: F, t2465: F) -> (F, F, F, F, F, F) {
    let t6965 = t6964 * t6706;
    let t6968 = t4399 * t2471;
    let t6970 = t487 * t1305;
    let t6971 = t2365 * t6970;
    let t6972 = t1416 * t6971;
    let t6974 = t4803 * t586;
    let t6975 = t6974 * t2479;
    let t6977 = t2465 * t1065;
    (t6965, t6968, t6972, t6974, t6975, t6977)
}
