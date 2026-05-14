//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 967/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk967<F: Float>(t1227: F, t937: F, t2363: F, t3199: F, t410: F, t3258: F, t6523: F, t7832: F, t8436: F, t2393: F, t133: F, t8309: F, t945: F, t2970: F, t8445: F, t6455: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8511 = t937 * t1227;
    let t8512 = t2363 * t8511;
    let t8515 = t410 * t3199;
    let t8516 = t2363 * t8515;
    let t8519 = t6523 * t3258;
    let t8520 = t7832 * t8436;
    let t8529 = t2393 * t8511;
    let t8532 = t8309 * t133;
    let t8533 = t8532 * t945;
    let t8536 = t2393 * t8515;
    let t8539 = t2970 * t8445;
    let t8542 = t6455 * t3258;
    (t8511, t8512, t8515, t8516, t8519, t8520, t8529, t8532, t8533, t8536, t8539, t8542)
}
