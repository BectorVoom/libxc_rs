//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 357/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk357(t1608: f64, t2104: f64, t286: f64, t1597: f64, t1599: f64, t2096: f64, t2100: f64, t619: f64) -> (f64, f64) {
    let t2105 = t1608 * t2104;
    let t2106 = t286 * t2105;
    let t2109 = -t2096 * t619 / 72.0_f64 + t1597 + t1599 * t2100 / 576.0_f64 - t1599 * t2106 / 192.0_f64;
    (t2105, t2109)
}
