//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 810/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk810<F: Float>(t11060: F, t11072: F, t11085: F, t11097: F, t11111: F, t11123: F, t11136: F, t11148: F, t1125: F, t2822: F, t3649: F, t423: F, t1459: F, t3652: F, t1423: F, t1464: F) -> (F, F, F, F, F, F) {
    let t11151 = t11060 + t11072 + t11085 + t11097 + t11111 + t11123 + t11136 + t11148;
    let t11155 = t1125 * t2822;
    let t11181 = t3649 * t423;
    let t11182 = t11181 * t1459;
    let t11183 = t11182 * t3652;
    let t11185 = t1423 * t1464;
    (t11151, t11155, t11181, t11182, t11183, t11185)
}
