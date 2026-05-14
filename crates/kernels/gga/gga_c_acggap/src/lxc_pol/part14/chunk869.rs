//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 869/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk869<F: Float>(t34052: F, t2304: F, t7780: F, t7799: F, t8545: F, t8491: F, t30402: F, t31309: F, t525: F, t7325: F, t31362: F, t8783: F, t4959: F, t7647: F, t30148: F, t5606: F, t7585: F, t7842: F) -> (F, F, F, F, F, F, F, F) {
    let t34053 = 0.7145669686344956162e-3 * t34052;
    let t34054 = t7780 * t2304;
    let t34056 = t7799 * t8545;
    let t34059 = t7799 * t8491;
    let t34068 = t31309 * t30402 * t7325 * t525;
    let t34081 = t31362 * t8783;
    let t34082 = 0.15724046144802076034e-2 * t34081;
    let t34091 = t7647 * t4959;
    let t34092 = 0.17149607247227894789e-2 * t34091;
    let t34095 = t7585 * t7842 * t30148 * t5606;
    (t34053, t34054, t34056, t34059, t34068, t34082, t34092, t34095)
}
