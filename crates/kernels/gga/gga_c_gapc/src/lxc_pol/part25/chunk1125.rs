//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1125/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1125<F: Float>(t28415: F, t8140: F, t1: F, t761: F, t2188: F, t3329: F, t959: F, t2619: F, t2972: F, t190: F, t2211: F, t154: F, t6182: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t28526 = t28415 * t8140;
    let t28594 = t761 * t1;
    let t28602 = pi * t2188 * t959 * t3329;
    let t28609 = t2619 * t2972;
    let t28622 = t2211 * t190;
    let t28920 = t154 * t6182;
    (t28526, t28594, t28602, t28609, t28622, t28920)
}
