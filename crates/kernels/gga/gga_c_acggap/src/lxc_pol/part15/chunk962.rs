//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 962/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk962<F: Float>(t7799: F, t8545: F, t30260: F, t8491: F, t30402: F, t31309: F, t525: F, t7325: F, t30273: F, t30280: F, t31362: F, t8783: F) -> (F, F, F, F, F, F, F) {
    let t34056 = t7799 * t8545;
    let t34058 = F::cast_from(0.13976929906490734252e-1_f64) * t30260;
    let t34059 = t7799 * t8491;
    let t34068 = t31309 * t30402 * t7325 * t525;
    let t34076 = F::cast_from(0.21437009059034868486e-3_f64) * t30273;
    let t34077 = F::cast_from(0.28582678745379824648e-3_f64) * t30280;
    let t34081 = t31362 * t8783;
    (t34056, t34058, t34059, t34068, t34076, t34077, t34081)
}
