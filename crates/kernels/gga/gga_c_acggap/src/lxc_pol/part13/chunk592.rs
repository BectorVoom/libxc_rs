//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 592/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk592<F: Float>(t535: F, t930: F, t1181: F, t1165: F, t540: F, t1490: F, t330: F, t3740: F, t527: F, t1017: F, t495: F, t1089: F, t1459: F) -> (F, F, F, F, F, F) {
    let t4330 = t535 * t930;
    let t4331 = t1181 * t4330;
    let t4335 = t1165 * t540 * t930;
    let t4339 = F::new(7.0) / F::new(144.0) * t330 * t1490;
    let t4340 = t3740 * t527;
    let t4342 = t495 * t1017;
    let t4344 = t1089 * t1459 * t4342;
    (t4331, t4335, t4339, t4340, t4342, t4344)
}
