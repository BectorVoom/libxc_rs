//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1097/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1097(t1720: f64, t7348: f64, t1190: f64, t8054: f64, t1751: f64, t7284: f64, t7287: f64, t1251: f64, t1409: f64, t24602: f64, t24601: f64, t1090: f64, t27381: f64) -> (f64, f64, f64, f64, f64) {
    let t27422 = t1720 * t7348;
    let t27424 = t1190 * t8054;
    let t27426 = t7284 * t1751;
    let t27427 = t27426 * t7287;
    let t27432 = t1409 * t1251;
    let t27433 = t24602 * t27432;
    let t27434 = t24601 * t27433;
    let t27437 = t27381 * t1090;
    (t27422, t27424, t27427, t27434, t27437)
}
