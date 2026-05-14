//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1027/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1027<F: Float>(t11513: F, t1749: F, t5285: F, t1743: F, t34123: F, t4979: F, t11449: F, t11519: F, t1845: F, t190: F, t11451: F, t11518: F, t20897: F, t11517: F, t33490: F, t34535: F, t5117: F) -> (F, F, F, F, F, F) {
    let t34644 = t5285 * t11513 * t1749;
    let t34647 = t1743 * t34123 * t4979;
    let t34651 = t1845 * t190 * t11449 * t11519;
    let t34654 = t11518 * t11451 * t20897;
    let t34656 = t11517 * t33490;
    let t34658 = t34656 * t34535 * t5117;
    (t34644, t34647, t34651, t34654, t34656, t34658)
}
