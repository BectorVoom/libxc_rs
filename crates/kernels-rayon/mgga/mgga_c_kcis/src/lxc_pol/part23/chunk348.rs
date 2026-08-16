//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 348/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk348(t2046: f64, t572: f64, t571: f64, t1981: f64, t552: f64, t577: f64, t585: f64, t2001: f64, t584: f64, t583: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2047 = t572 * t2046;
    let t2048 = t571 * t2047;
    let t2050 = t1981 * t552;
    let t2051 = t2050 * t577;
    let t2052 = t2051 * t585;
    let t2054 = t584 * t2001;
    let t2055 = t583 * t2054;
    (t2047, t2048, t2050, t2051, t2052, t2055)
}
