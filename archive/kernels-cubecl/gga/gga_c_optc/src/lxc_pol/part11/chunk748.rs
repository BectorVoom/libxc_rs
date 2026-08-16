//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 748/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk748<F: Float>(t1245: F, t2048: F, t1983: F, t3305: F, t3399: F, t539: F, t544: F, t1264: F, t658: F, t1: F, t6855: F, t1274: F, t6893: F) -> (F, F, F, F, F, F, F) {
    let t9705 = t2048 * t1245;
    let t9707 = t3305 * t1983;
    let t9715 = t539 * t3399;
    let t9721 = t544 * t3399;
    let t9735 = t1264 * t658;
    let t9747 = t6855 * t1;
    let t9769 = t6893 * t1274;
    (t9705, t9707, t9715, t9721, t9735, t9747, t9769)
}
