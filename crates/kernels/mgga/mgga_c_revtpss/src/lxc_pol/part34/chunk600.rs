//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 600/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk600<F: Float>(t1082: F, t6244: F, t1089: F, t6271: F, t1651: F, t5004: F, t6258: F, t378: F, t6305: F, t3304: F, t1668: F, t1678: F, t6299: F, t3318: F, t380: F, t6343: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6362 = t1082 * t6244;
    let t6365 = t6271 * t1089;
    let t6368 = t5004 * t1651;
    let t6371 = t1082 * t6258;
    let t6374 = t378 * t6305;
    let t6375 = t6374 * t3304;
    let t6379 = t1678 * t1668 * t1089;
    let t6383 = t378 * t6299 * t1089;
    let t6386 = t6374 * t3318;
    let t6389 = t380 * t6343;
    (t6362, t6365, t6368, t6371, t6375, t6379, t6383, t6386, t6389)
}
