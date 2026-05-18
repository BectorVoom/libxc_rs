//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1033/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1033<F: Float>(t2318: F, t31261: F, t7538: F, t8689: F, t1352: F, t7746: F, t1967: F, t8486: F, t7736: F, t2450: F, t31349: F, t4469: F) -> (F, F, F, F, F, F, F) {
    let t34392 = t31261 * t2318;
    let t34394 = t7538 * t8689;
    let t34396 = t7746 * t1352;
    let t34398 = t1967 * t8486;
    let t34400 = t7736 * t1352;
    let t34406 = t2450 * t31349;
    let t34407 = t34406 * t4469;
    (t34392, t34394, t34396, t34398, t34400, t34406, t34407)
}
