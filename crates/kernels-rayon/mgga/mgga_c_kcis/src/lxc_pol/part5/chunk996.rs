//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 996/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk996(t12825: f64, t1602: f64, t1599: f64, t25: f64, t3977: f64, t209: f64, t494: f64, t617: f64, t736: f64, t612: f64, t110: f64, t1611: f64) -> (f64, f64, f64, f64) {
    let t12826 = t12825 * t1602;
    let t12827 = t1599 * t12826;
    let t12832 = t25 * t3977;
    let t12838 = t209 * t736 * t494 * t617;
    let t12840 = 5.0_f64 / 2592.0_f64 * t612 * t12838;
    let t12841 = t110 * t1611;
    (t12827, t12832, t12840, t12841)
}
