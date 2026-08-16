//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1196/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1196(t7968: f64, t95006: f64, t12651: f64, t1616: f64, t12581: f64, t1598: f64, t251: f64, t1370: f64, t27614: f64, t27664: f64, t4425: f64, t7978: f64) -> (f64, f64, f64, f64, f64) {
    let t95007 = t7968 * t95006;
    let t95009 = t12651 * t1616;
    let t95021 = t12581 * t251 * t1598;
    let t95024 = t1370 * t27614;
    let t95042 = t7978 * t4425 * t27664;
    (t95007, t95009, t95021, t95024, t95042)
}
