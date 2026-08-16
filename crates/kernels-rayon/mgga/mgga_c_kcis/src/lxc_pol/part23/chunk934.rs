//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 934/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk934(t1555: f64, t6048: f64, t4189: f64, t2069: f64, t4310: f64, t17253: f64, t552: f64, t577: f64, t585: f64, t1489: f64, t5880: f64, t4293: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17323 = t6048 * t1555;
    let t17325 = 4.0_f64 * t4189 * t17323;
    let t17326 = t2069 * t4310;
    let t17328 = 2.0_f64 * t4189 * t17326;
    let t17329 = t17253 * t552;
    let t17330 = t17329 * t577;
    let t17331 = t17330 * t585;
    let t17333 = t5880 * t1489;
    let t17334 = t4293 * t17333;
    (t17323, t17325, t17326, t17328, t17329, t17331, t17333, t17334)
}
