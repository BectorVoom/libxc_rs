//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 809/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk809<F: Float>(t10136: F, t10170: F, t10200: F, t10240: F, t10283: F, t10323: F, t10370: F, t10405: F, t3649: F, t423: F, t1459: F, t3652: F, t1423: F, t1464: F, t3651: F, t632: F, t996: F) -> (F, F, F, F, F, F, F) {
    let t10408 = t10136 + t10170 + t10200 + t10240 + t10283 + t10323 + t10370 + t10405;
    let t11181 = t3649 * t423;
    let t11182 = t11181 * t1459;
    let t11183 = t11182 * t3652;
    let t11185 = t1423 * t1464;
    let t11186 = t3651 * t11185;
    let t11188 = t996 * t632;
    (t10408, t11181, t11182, t11183, t11185, t11186, t11188)
}
