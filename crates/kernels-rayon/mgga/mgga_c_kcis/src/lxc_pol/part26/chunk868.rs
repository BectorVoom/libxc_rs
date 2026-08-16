//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 868/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk868(t20873: f64, t4162: f64, t4160: f64, t1497: f64, t4171: f64, t6284: f64, t4170: f64, t833: f64) -> (f64, f64, f64, f64) {
    let t20874 = t4162 * t20873;
    let t20875 = t4160 * t20874;
    let t20878 = t4171 * t6284 * t1497;
    let t20879 = t4170 * t20878;
    let t20880 = t4160 * t20879;
    let t20882 = t6284 * t833;
    (t20875, t20878, t20880, t20882)
}
