//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 729/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk729<F: Float>(t1426: F, t368: F, t8539: F, t598: F, t1479: F, t7476: F, t1980: F, t1095: F, t1988: F, t2304: F, t1089: F, t2302: F, t3201: F, t137: F, t1487: F, t1083: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8541 = t1426 * t368 * t8539;
    let t8542 = t598 * t8541;
    let t8544 = t368 * t1479;
    let t8545 = t7476 * t8544;
    let t8546 = t1980 * t8545;
    let t8549 = t1426 * t1095 * t8539;
    let t8550 = t598 * t8549;
    let t8555 = t7476 * t1095 * t1479;
    let t8556 = t1980 * t8555;
    let t8558 = t1988 * t2304;
    let t8561 = t1089 * t3201 * t2302;
    let t8562 = t598 * t8561;
    let t8564 = t137 * t1487;
    let t8566 = t1089 * t1083 * t8564;
    (t8541, t8542, t8544, t8545, t8546, t8549, t8550, t8555, t8556, t8558, t8561, t8562, t8564, t8566)
}
