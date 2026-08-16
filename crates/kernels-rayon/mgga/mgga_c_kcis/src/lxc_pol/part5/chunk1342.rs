//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1342/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1342(t16359: f64, t21625: f64, t1319: f64, t16582: f64, t22114: f64, t3255: f64, t7222: f64, t3780: f64, t531: f64, t1650: f64, t1897: f64, t11634: f64, t1419: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22184 = t16359 * t21625;
    let t22188 = t16582 * t22114 * t1319;
    let t22191 = t3255 * t7222;
    let t22193 = t3780 * t531;
    let t22194 = t1650 * t1897;
    let t22196 = t22193 * t22194 * t1319;
    let t22200 = t11634 * t22194 * t1419;
    (t22184, t22188, t22191, t22194, t22196, t22200)
}
