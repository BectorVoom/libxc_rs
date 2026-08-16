//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 708/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk708(t5752: f64, t585: f64, t1468: f64, t2055: f64, t1395: f64, t2062: f64, t2066: f64, t8189: f64, t8192: f64, t8194: f64, t8197: f64) -> (f64, f64, f64, f64, f64) {
    let t8199 = t5752 * t585;
    let t8201 = t1468 * t2055;
    let t8203 = t1395 * t2062;
    let t8205 = t1395 * t2066;
    let t8207 = t8189 / 16.0_f64 - t8192 / 16.0_f64 - t8194 / 6.0_f64 + t8197 / 24.0_f64 - t8199 / 128.0_f64 + t8201 / 128.0_f64 + t8203 / 24.0_f64 - t8205 / 96.0_f64;
    (t8199, t8201, t8203, t8205, t8207)
}
