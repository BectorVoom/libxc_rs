//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1143/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1143(t1704: f64, t4621: f64, t14546: f64, t1003: f64, t6330: f64, t2894: f64, t18570: f64, t4947: f64, t14554: f64, t18574: f64, t6334: f64, t18677: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19218 = t4621 * t1704;
    let t19219 = t14546 * t19218;
    let t19222 = t6330 * t1003;
    let t19223 = t2894 * t19222;
    let t19226 = t4947 * t18570;
    let t19229 = t14554 * t18574;
    let t19232 = t6334 * t1003;
    let t19233 = t2894 * t19232;
    let t19236 = t4947 * t18677;
    (t19219, t19223, t19226, t19229, t19233, t19236)
}
