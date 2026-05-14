//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 323/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk323<F: Float>(t1423: F, t568: F, t423: F, t61: F, t472: F, t457: F, t567: F) -> (F, F, F, F) {
    let t1424 = t1423 * t568;
    let t1427 = t61 * t423;
    let t1428 = t1427 * t472;
    let t1431 = t457 * t567;
    (t1424, t1427, t1428, t1431)
}
