//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1031/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1031(t12838: f64, t612: f64, t110: f64, t1611: f64, t1599: f64, t1607: f64, t3970: f64, t4441: f64, t4439: f64, t4425: f64, t4446: f64, t1590: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12840 = 5.0_f64 / 2592.0_f64 * t612 * t12838;
    let t12841 = t110 * t1611;
    let t12842 = t1599 * t12841;
    let t12844 = t3970 * t1607;
    let t12845 = t12844 * t4441;
    let t12846 = t4439 * t12845;
    let t12848 = t4425 * t4446;
    let t12849 = t1599 * t12848;
    let t12856 = t1590 * t1590;
    (t12840, t12842, t12844, t12846, t12849, t12856)
}
