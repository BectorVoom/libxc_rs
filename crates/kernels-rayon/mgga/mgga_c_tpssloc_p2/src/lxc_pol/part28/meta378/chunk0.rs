//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1445/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1445(t1147: f64, t1156: f64, t14829: f64, t1164: f64, t3423: f64, t4869: f64, t11126: f64, t1703: f64, t1657: f64, t3263: f64, t3266: f64, t11292: f64, t1694: f64) -> (f64, f64, f64, f64, f64) {
    let t14831 = t1147 * t14829 * t1156;
    let t14833 = 0.5848223622634646207e0_f64 * t1164 * t14831;
    let t14835 = 0.17315859105681463759e2_f64 * t4869 * t3423;
    let t14837 = 0.5848223622634646207e0_f64 * t11126 * t1703;
    let t14838 = t1657 * t3263;
    let t14840 = 2.0_f64 * t14838 * t3266;
    let t14841 = t11292 * t1694;
    (t14833, t14835, t14837, t14840, t14841)
}
