//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1139/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1139(t4958: f64, t984: f64, t4943: f64, t9938: f64, t991: f64, t2880: f64, t291: f64, t4581: f64, t9959: f64, t4567: f64, t2469: f64, t992: f64) -> (f64, f64, f64, f64, f64) {
    let t14439 = t984 * t4958 / 54.0_f64;
    let t14440 = t9938 * t4943;
    let t14442 = t991 * t14440 / 432.0_f64;
    let t14443 = t2880 * t291;
    let t14444 = t14443 * t4581;
    let t14446 = t991 * t14444 / 216.0_f64;
    let t14447 = t9959 * t291;
    let t14448 = t14447 * t4567;
    let t14450 = t991 * t14448 / 324.0_f64;
    let t14453 = t2469 * t992;
    (t14439, t14442, t14446, t14450, t14453)
}
