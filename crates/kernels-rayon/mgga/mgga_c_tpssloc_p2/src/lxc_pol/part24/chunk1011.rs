//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1011/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1011(t11399: f64, t1147: f64, t1156: f64, t1164: f64, t3411: f64, t3419: f64, t3423: f64, t11203: f64, t11206: f64, t11209: f64, t11211: f64, t11213: f64, t11215: f64, t11217: f64, t11221: f64, t11224: f64, t11230: f64, t11233: f64) -> (f64, f64, f64, f64) {
    let t11478 = t1147 * t11399 * t1156;
    let t11480 = 0.5848223622634646207e0_f64 * t1164 * t11478;
    let t11482 = 0.17544670867903938621e1_f64 * t3411 * t3419;
    let t11484 = 0.51947577317044391276e2_f64 * t3411 * t3423;
    let t11487 = 20.0_f64 / 27.0_f64 * t11203;
    let t11496 = t11487 - 5.0_f64 / 9.0_f64 * t11211 - t11213 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t11215 + t11217 / 3.0_f64 - 2.0_f64 / 27.0_f64 * t11221 + t11224 / 3.0_f64 + t11230 / 6.0_f64 - t11206 - t11233 - t11209 / 6.0_f64;
    (t11480, t11482, t11484, t11496)
}
