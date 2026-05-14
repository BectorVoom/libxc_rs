//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 758/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk758<F: Float>(t43657: F, t43660: F, t43679: F, t43681: F, t11848: F, t2021: F, t7372: F, t11576: F, t123: F, t883: F, t2684: F, t2685: F, t11608: F, t2464: F, t2465: F, t2365: F, t35550: F, t7630: F) -> (F, F, F, F, F, F, F, F, F) {
    let t45454 = 0.20449560508757733161e1 * t43657;
    let t45457 = 0.34082600847929555269e0 * t43660;
    let t45458 = 0.59584149919750711116e-1 * t43679;
    let t45459 = 0.71500979903700853339e0 * t43681;
    let t45463 = t2021 * t11848 * t7372;
    let t45464 = 0.14896037479937677779e-1 * t45463;
    let t45466 = t11576 * t123 * t883;
    let t45468 = t2684 * t2685 * t45466;
    let t45469 = 0.19171462976960374838e0 * t45468;
    let t45472 = t2684 * t2464 * t2465 * t11608;
    let t45473 = 0.42603251059911944084e-1 * t45472;
    let t45475 = t7630 * t2365 * t35550;
    (t45454, t45457, t45458, t45459, t45464, t45466, t45469, t45473, t45475)
}
