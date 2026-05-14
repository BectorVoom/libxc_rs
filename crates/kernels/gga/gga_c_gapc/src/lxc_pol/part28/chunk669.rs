//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 669/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk669<F: Float>(t2530: F, t350: F, t311: F, t6851: F, t442: F, t6856: F, t277: F, t4978: F, t2188: F, t329: F, t314: F, t2387: F, t825: F, t2389: F, t2763: F, t327: F) -> (F, F, F, F, F, F, F, F) {
    let t7949 = t2530 * t350;
    let t7953 = t311 * t6851;
    let t7956 = t6856 * t442;
    let t7967 = t277 * t4978;
    let t7974 = t2188 * t329;
    let t7975 = t7974 * t314;
    let t8061 = t2387 * t825;
    let t8117 = t2389 * t825;
    let t8131 = t327 * t2763;
    (t7949, t7953, t7956, t7967, t7975, t8061, t8117, t8131)
}
