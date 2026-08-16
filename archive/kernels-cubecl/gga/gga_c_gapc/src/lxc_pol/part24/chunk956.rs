//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 956/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk956<F: Float>(t3679: F, t5248: F, t1643: F, t3683: F, t424: F, t205: F, t3680: F, t5252: F, t3091: F, t3670: F, t19: F, t515: F) -> (F, F, F, F, F, F, F) {
    let t11566 = t3679 * t5248;
    let t11567 = t1643 * t11566;
    let t11569 = t424 * t3683;
    let t11570 = t11569 * t205;
    let t11572 = t5252 * t3680;
    let t11574 = t3670 * t3091;
    let t11576 = t515 * t19;
    (t11566, t11567, t11569, t11570, t11572, t11574, t11576)
}
