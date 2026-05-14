//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 864/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk864<F: Float>(t1165: F, t14575: F, t604: F, t7346: F, t1089: F, t31520: F, t31521: F, t368: F, t1198: F, t2095: F, t355: F, t151: F, t7731: F, t950: F, t947: F, t7685: F, t932: F) -> (F, F, F, F, F) {
    let t31797 = t7346 * t1165 * t604 * t14575;
    let t31805 = t31520 * t1089 * t368 * t31521;
    let t31808 = t2095 * t1198 * t355;
    let t31811 = t151 * t7731 * t950;
    let t31812 = t31811 * t947;
    let t31816 = t7685 * t932;
    (t31797, t31805, t31808, t31812, t31816)
}
