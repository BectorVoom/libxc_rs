//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 786/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk786<F: Float>(t9328: F, t9330: F, t2993: F, t3127: F, t5392: F, t3132: F, t5395: F, t3128: F, t5633: F, t3133: F, t633: F, t8992: F) -> (F, F, F, F, F, F) {
    let t9331 = t9328 * t9330;
    let t9333 = t2993 * t3127;
    let t9334 = t9333 * t5392;
    let t9336 = t5395 * t3132;
    let t9337 = t9336 * t5392;
    let t9339 = t3128 * t5633;
    let t9341 = t3133 * t5633;
    let t9343 = t633 * t8992;
    (t9331, t9334, t9337, t9339, t9341, t9343)
}
