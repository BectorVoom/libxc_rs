//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 911/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk911<F: Float>(t137: F, t4099: F, t1426: F, t368: F, t598: F, t4806: F, t1980: F, t7476: F, t2304: F, t7780: F, t7799: F, t8545: F, t30260: F, t8491: F, t336: F, t4838: F, t578: F, t599: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34045 = t137 * t4099;
    let t34048 = t598 * t1426 * t368 * t34045;
    let t34050 = t368 * t4806;
    let t34052 = t1980 * t7476 * t34050;
    let t34053 = 0.7145669686344956162e-3 * t34052;
    let t34054 = t7780 * t2304;
    let t34056 = t7799 * t8545;
    let t34058 = 0.13976929906490734252e-1 * t30260;
    let t34059 = t7799 * t8491;
    let t34063 = t578 * t336 * t599 * t4838;
    (t34045, t34048, t34050, t34053, t34054, t34056, t34058, t34059, t34063)
}
