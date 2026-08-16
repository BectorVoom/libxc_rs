//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 615/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk615(t3999: f64, t494: f64, t450: f64, t740: f64, t518: f64, t1405: f64, t532: f64, t1401: f64, t1420: f64, t1444: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4000 = t494 * t3999;
    let t4016 = t740 * t450;
    let t4018 = 0.46853067927761790996e-2_f64 * t4016 * t518;
    let t4019 = t532 * t1405;
    let t4021 = t1401 * t1420;
    let t4023 = t89 * t1444;
    (t4000, t4016, t4018, t4019, t4021, t4023)
}
