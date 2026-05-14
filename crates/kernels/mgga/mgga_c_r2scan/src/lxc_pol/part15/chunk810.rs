//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 810/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk810<F: Float>(t259: F, t2649: F, t546: F, t565: F, t1551: F, t2562: F, t360: F, t2526: F, t537: F, t2124: F, t495: F, t2719: F, t277: F, t2567: F, t1567: F, t2530: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7983 = t2649 * t259;
    let t7984 = t546 * t7983;
    let t7987 = t565 * t7983;
    let t7990 = t2562 * t1551;
    let t7991 = t360 * t7990;
    let t7994 = t537 * t2526;
    let t7996 = t2124 * t7994 * t495;
    let t8001 = t277 * t2719;
    let t8002 = t8001 * t495;
    let t8003 = t360 * t8002;
    let t8006 = t2567 * t1551;
    let t8007 = t360 * t8006;
    let t8012 = t1567 * t2530;
    (t7984, t7987, t7990, t7991, t7996, t8001, t8002, t8003, t8006, t8007, t8012)
}
