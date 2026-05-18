//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1053/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1053<F: Float>(t158: F, t6393: F, t1: F, t544: F, t2371: F, t4461: F, t1397: F, t6699: F, t4370: F, t4389: F, t1457: F, t2378: F) -> (F, F, F, F, F) {
    let t21069 = t158 * t6393;
    let t21071 = t544 * t21069 * t1;
    let t21074 = t4461 * t2371;
    let t21077 = t1397 * t6699;
    let t21133 = t544 * t4389 * t4370;
    let t21139 = t1457 * t2378;
    (t21071, t21074, t21077, t21133, t21139)
}
