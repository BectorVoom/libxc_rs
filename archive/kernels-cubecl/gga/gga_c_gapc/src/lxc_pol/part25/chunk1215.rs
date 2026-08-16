//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1215/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1215<F: Float>(t16471: F, t2553: F, t30324: F, t3402: F, t519: F, t6: F, t1084: F, t11927: F, t1461: F, t291: F, t8709: F, t1971: F, t818: F, t8448: F, t9846: F) -> (F, F, F) {
    let t34264 = t3402 * t519 * t16471 * t2553 * t6 * t30324;
    let t34269 = t1084 * t1461 * t8709 * t291 * t11927;
    let t34274 = t1084 * t1971 * t8448 * t818 * t9846;
    (t34264, t34269, t34274)
}
