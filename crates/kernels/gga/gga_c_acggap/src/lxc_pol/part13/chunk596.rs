//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 596/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk596<F: Float>(t1165: F, t540: F, t955: F, t535: F, t1181: F, t530: F, t1532: F, t1016: F, t513: F) -> (F, F, F, F, F) {
    let t4402 = t1165 * t540 * t955;
    let t4405 = t535 * t955;
    let t4406 = t1181 * t4405;
    let t4410 = t1165 * t530 * t955;
    let t4414 = t1165 * t1532 * t955;
    let t4417 = t1016 * t513;
    (t4402, t4406, t4410, t4414, t4417)
}
