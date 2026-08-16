//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 688/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk688<F: Float>(t2343: F, t6443: F, t2293: F, t555: F, t494: F, t2312: F, t2327: F, t4245: F, t883: F, t485: F, t1320: F, t481: F, t880: F) -> (F, F, F, F, F, F) {
    let t6444 = t2343 * t6443;
    let t6447 = t555 * t2293;
    let t6448 = t6447 * t494;
    let t6451 = t2312 * t2327;
    let t6455 = t883 * t4245;
    let t6456 = t485 * t6455;
    let t6457 = t481 * t880 * t1320 * t6456;
    (t6444, t6447, t6448, t6451, t6455, t6457)
}
