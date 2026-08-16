//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 803/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk803(t110: f64, t1369: f64, t1602: f64, t1599: f64, t4425: f64, t4450: f64, t25: f64, t3977: f64, t4434: f64, t209: f64, t494: f64, t617: f64, t736: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12825 = t110 * t1369;
    let t12826 = t12825 * t1602;
    let t12827 = t1599 * t12826;
    let t12829 = t4425 * t4450;
    let t12830 = t1599 * t12829;
    let t12832 = t25 * t3977;
    let t12833 = t12832 * t4434;
    let t12834 = t1599 * t12833;
    let t12838 = t209 * t736 * t494 * t617;
    (t12825, t12827, t12830, t12832, t12834, t12838)
}
