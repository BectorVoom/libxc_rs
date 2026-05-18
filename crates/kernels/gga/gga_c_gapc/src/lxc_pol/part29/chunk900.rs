//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 900/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk900<F: Float>(t10136: F, t10170: F, t10200: F, t10240: F, t10283: F, t10323: F, t10370: F, t10405: F, t3649: F, t423: F, t1459: F, t3652: F) -> (F, F, F, F) {
    let t10408 = t10136 + t10170 + t10200 + t10240 + t10283 + t10323 + t10370 + t10405;
    let t11181 = t3649 * t423;
    let t11182 = t11181 * t1459;
    let t11183 = t11182 * t3652;
    (t10408, t11181, t11182, t11183)
}
