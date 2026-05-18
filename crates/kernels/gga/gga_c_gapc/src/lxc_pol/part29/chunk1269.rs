//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1269/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1269<F: Float>(t11249: F, t1459: F, t25176: F, t11215: F, t13676: F, t13679: F, t520: F, t11216: F, t13646: F, t13654: F, t35541: F, t3948: F) -> (F, F, F, F) {
    let t35643 = t25176 * t1459 * t11249;
    let t35647 = t11215 * t13676 * t520 * t13679;
    let t35650 = t11216 * t520 * t13646;
    let t35653 = t35541 * t3948 * t13654;
    (t35643, t35647, t35650, t35653)
}
